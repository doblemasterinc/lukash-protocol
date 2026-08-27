# Spec 07-f — KASH Shield: Anti-Whale + KASH Exit Fee sobre Token-2022 Transfer Hook

> Milestone 2 · Referencia: Protocolo v4.3 §9 (Anti-Whale + Exit Fee) + ADR-012 (C10) + ADR-015 · Prioridad: **alta**
> Dependencias upstream: 07-a (Pyth), 07-c (Tridente para registro de MMs).
> **Cambia el tipo de mint del token en mainnet** — decisión estratégica de Sebastián (ADR-015).

---

## 1. Problema (qué hay hoy)

Milestone 1 tiene el token $LUKA en devnet como **SPL clásico**
(`2DatjaKezpYkB3TitgwYGvpwTAWiFxN4JEwpYnk3Luvr`, ver `RUNBOOK_DEVNET.md`). Con SPL
clásico, **cualquier fee anti-dump que apliquemos en el contrato solo funciona
para transacciones que pasan por el contrato**. Un ballena que quiera evadir el
Anti-Whale/Exit Fee simplemente vende en otro DEX (Raydium, Orca) y esquiva
completamente el fee.

Para un token cuyo value prop es "cada venta de pánico fortalece la Reserva"
(KASH Exit Fee), un enforcement evadible es **casi inútil**. El diseño del
KASH Shield asume que el fee se aplica **en toda transferencia**, no solo en
las nuestras.

Además, la lógica del Anti-Whale (ADR-012 / C10) y Exit Fee (v4.3 §9) **no está
implementada** en `lib.rs` — es Milestone 2.

## 2. Diseño

### 2.1 Mint del token — Token-2022 con Transfer Hook (aprobado ADR-015)

**Cambio estructural del mint en mainnet.** El mint devnet clásico se mantiene solo
para pruebas Milestone 1 y 2 (que no requieren enforcement anti-dump on-chain).
El mint de mainnet será **Token-2022** con las siguientes extensiones activadas:

| Extensión Token-2022 | Uso |
| --- | --- |
| **Transfer Hook** | Redirige cada transferencia al programa LUKASH para aplicar Anti-Whale + Exit Fee |
| **Metadata Pointer** | Nombre, logo, URI (ya usado en devnet vía Metaplex, aquí es nativo) |
| **Mint Close Authority = disabled** | Nadie puede cerrar el mint |
| ~~Transfer Fee~~ | **NO se usa** — el fee es dinámico según Aura/tamaño/precio, no aplica el Transfer Fee estático de Token-2022 |
| ~~Confidential Transfer~~ | No; queremos observabilidad on-chain para verificación |

**Compatibilidad verificada:** Jupiter y Meteora soportan Token-2022 con transfer
hooks desde 2024. Kamino y Marginfi también. CEXs: Kraken, Coinbase y Binance
listan Token-2022; los tier 2 varían — verificación pre-listing por parte del
equipo.

### 2.2 Arquitectura del Transfer Hook

**Programa hook = el mismo programa `lukash_protocol`** (una sola dirección para
todo). Nueva instrucción `transfer_hook` que Token-2022 invoca automáticamente en
cada transferencia:

```rust
pub fn transfer_hook(
    ctx: Context<TransferHook>,
    amount: u64,           // tokens siendo transferidos (unidades nativas)
) -> Result<()> {
    // 1. Cargar el estado del protocolo (config + state)
    // 2. Cargar el registro del sender (para evaluar Aura, staking, LP lock, etc.)
    // 3. Determinar si la tx es "venta al pool" (Meteora), "swap interno D",
    //    "transferencia peer-to-peer", o "compra desde el pool"
    // 4. Aplicar reglas:
    //      - Si es COMPRA (from pool): NO Anti-Whale ni Exit Fee (ADR-012 C10)
    //      - Si el sender está EXENTO (§2.4): NO fee adicional
    //      - Si aplica Anti-Whale por tamaño → agregar fee sobre excedente
    //      - Si aplica Exit Fee por pánico → agregar fee sobre monto total
    //      - Enviar fees calculados al Vault Core (100% al Vault — ADR-012)
    // 5. Emitir eventos AntiWhaleTriggered / ExitFeeTriggered / TransferInspected
}
```

**Cómo se aplica el fee: cobro atómico directo.** El hook retiene el fee del monto
transferido en la misma transacción. El sender recibe `amount - fee_total` y el
`fee_total` se transfiere al ATA del Vault Core en la misma tx.

```rust
// Dentro del transfer_hook, después de calcular fee_tokens:
let net_amount = amount.checked_sub(fee_tokens).ok_or(LukashError::MathOverflow)?;
// Token-2022 entrega net_amount al destinatario
// fee_tokens → CPI transfer al Vault Core ATA (USDC vía Jupiter swap inline)
transfer_fee_to_vault_cpi(ctx, fee_tokens)?;
```

**Ventajas vs modo deuda (descartado):** (1) sin wallets desechables que acumulen
deuda incobrable; (2) sin instrucción `collect_whale_debt` adicional; (3) el fee
se cobra garantizado, no depende de un keeper posterior. **Trade-off:** el usuario
recibe menos tokens de los que espera en el swap — el frontend debe mostrar el
fee estimado pre-tx (igual que cualquier DEX muestra slippage).

### 2.3 Anti-Whale (ADR-012 C10) — reglas

**Umbral por % del pool de liquidez** (no supply — corrección v4.2 → v4.3).

En el hook, cuando la tx es identificada como venta al pool:

```rust
let pool_liquidity_usd = meteora_pool_liquidity(ctx)?;
let tx_usd = amount as u128 * oracle_price_luka as u128 / SCALE;
let tx_pct_of_pool_bps = (tx_usd * BPS_DENOMINATOR as u128) / pool_liquidity_usd as u128;

let (excedente_bps, penal_bps) = match tx_pct_of_pool_bps as u64 {
    b if b < 100  => (0, 0),                   // <1% del pool: sin fee
    b if b <= 200 => (b - 100, 300),           // 1-2%: 3% sobre excedente
    b if b <= 500 => (b - 100, 600),           // 2-5%: 6% sobre excedente
    _             => (tx_pct_of_pool_bps as u64 - 100, 1000),  // >5%: 10% sobre excedente
};

let excedente_tokens = mul_bps(amount, excedente_bps);
let fee_tokens = mul_bps(excedente_tokens, penal_bps);
// fee_tokens → cobro atómico directo → swap LUKA→USDC → Vault Core
```

**Exención por Aura ≥ 25,000 (nivel Titán) — override Anti-Whale:** requiere leer
el registro Aura del sender (existente en el diseño, PDA por wallet). Si `aura ≥ 25_000`,
se salta la penalización. Esto además libera de escala automática a Circuit Breaker LP.

### 2.4 KASH Exit Fee (v4.3 §9) — activación dual

Se activa cuando **ambas** condiciones se cumplen:

```rust
let ema30 = state.ema30;  // ya existente
let luka_price = state.luka_price;  // 07-a
let cond_price = luka_price < mul_bps(ema30, 7_000);  // <0.7×EMA30

// Volumen de venta sobre supply — necesitamos una ventana rodante de 1h
// Nuevo estado: sell_pressure_1h_supply_bps + last_reset_ts
let cond_volume = state.sell_pressure_1h_supply_bps > 30;  // >0.3% supply/hora

if cond_price && cond_volume {
    // Aplicar Exit Fee
    let fee_bps = match config.stage {
        1 => 500,   // Génesis 5%
        2 => 300,   // Etapa 2 3%
        _ => 100,   // Etapa 3+ 1%
    };
    let fee_tokens = mul_bps(amount, fee_bps);
    // → cobro atómico directo → swap LUKA→USDC → Vault Core (100% ADR-012)
}
```

### 2.5 Exenciones canónicas (ADR-015 punto 6)

Verificación en el hook, en orden de barato → caro (fail-fast):

```rust
fn is_exempt(sender: Pubkey, ctx: &Context<TransferHook>) -> Result<bool> {
    // 1. Swap interno Motor D — si el CPI viene desde el propio programa lukash_protocol
    if ctx.accounts.instructions.load()?.program_id == crate::ID {
        return Ok(true);
    }
    // 2. Sender en la lista de MMs registrados en Tridente Multisig
    if is_registered_mm(ctx, sender)? { return Ok(true); }
    // 3. LP Fundador 365d — sender es el ATA del LP Fundador
    if sender == config.lp_fundador_ata { return Ok(true); }
    // 4. LP Comprometido en lock activo — sender tiene una posición activa
    if has_active_lp_lock(ctx, sender)? { return Ok(true); }
    // 5. Staking activo — sender tiene una posición staking > 0
    if has_active_staking(ctx, sender)? { return Ok(true); }
    // 6. Nivel Titán (Aura ≥ 25_000)
    if get_aura_level(ctx, sender)? >= 25_000 { return Ok(true); }
    Ok(false)
}
```

**KOLs: NO están en esta lista.** El mecanismo que los alinea es el vesting on-chain
(ADR-011) — no una exención estática. Si un KOL logra vender por encima del umbral
(improbable con vesting escalonado), el Anti-Whale sí aplica.

**Nota sobre la exención de staking sin umbral mínimo (decisión consciente):**
La exención por staking activo no exige un monto mínimo stakeado. Un holder podría
stakear una cantidad trivial y calificar. Se acepta este trade-off porque: (1) el fee
escalonado del Anti-Whale (3%/6%/10%) ya es la protección principal — el incentivo
económico disuade ventas masivas independientemente de exenciones; (2) el Exit Fee
(pánico real) NO se exime por staking, así que las ventas en crisis siguen penalizadas;
(3) añadir un umbral incrementa el compute del hook por tx sin beneficio proporcional.
Si en producción se observa gaming, el umbral puede añadirse como parámetro en
`ProtocolConfig` ajustable vía timelock (48h), sin cambiar el contrato.

### 2.6 Registro de MMs — infraestructura nueva

Nueva instrucción `register_market_maker(mm_pubkey)`:

```rust
pub fn register_market_maker(
    ctx: Context<AdminOnly>,
    mm_pubkey: Pubkey,
) -> Result<()> {
    // Solo Tridente 3-de-3 puede registrar (ADR-015)
    assert_tridente_signed(&ctx.accounts.config, ctx.remaining_accounts)?;
    // Crea PDA MMRegistry con `is_active = true`
    // Emite MarketMakerRegistered
}

pub fn revoke_market_maker(...) -> Result<()> {
    // Igual pero flip `is_active = false`
}
```

**Cuenta PDA:** `["mm_registry", mm_pubkey]`. Consulta O(1) desde el hook.

### 2.7 Devnet vs mainnet — plan de transición

- **Milestone 2 devnet:** el hook se implementa y prueba en **red aparte** con
  un mint Token-2022 dedicado para tests (no reutilizar el mint SPL clásico —
  son formatos incompatibles).
- **Milestone 2 mainnet:** deploy del programa + acuñación del mint Token-2022
  productivo. El mint clásico devnet se abandona.
- **CEXs listing:** validación previa Jupiter + Meteora + al menos 1 CEX antes
  del TGE.

---

## 3. Contrato — interfaces

### 3.1 Constantes nuevas

```rust
// Anti-Whale (bps del pool)
pub const AW_THR_1_BPS: u64 = 100;   // 1%
pub const AW_THR_2_BPS: u64 = 200;   // 2%
pub const AW_THR_3_BPS: u64 = 500;   // 5%
pub const AW_FEE_1_BPS: u64 = 300;   // 3%
pub const AW_FEE_2_BPS: u64 = 600;   // 6%
pub const AW_FEE_3_BPS: u64 = 1_000; // 10%

// Exit Fee (bps por etapa)
pub const EXIT_FEE_ET1_BPS: u64 = 500;
pub const EXIT_FEE_ET2_BPS: u64 = 300;
pub const EXIT_FEE_ET3_BPS: u64 = 100;

// Activación Exit Fee
pub const EXIT_FEE_PRICE_TRIG_BPS: u64 = 7_000;  // <0.7×EMA30
pub const EXIT_FEE_VOL_TRIG_BPS:   u64 = 30;     // >0.3% supply/hora

// Aura Titán
pub const AURA_TITAN_MIN: u64 = 25_000;
```

### 3.2 Estado nuevo

```rust
// En ProtocolState
pub sell_pressure_1h_supply_bps: u64,   // rolling: bps de supply vendidos en última hora
pub sell_pressure_last_reset_ts: i64,

// Nueva PDA: MMRegistry (por mm_pubkey)
#[account]
pub struct MMRegistry {
    pub mm: Pubkey,
    pub is_active: bool,
    pub registered_at: i64,
    pub bump: u8,
}
```

### 3.3 Instrucciones nuevas

| Instrucción | Firmante | Función |
| --- | --- | --- |
| `transfer_hook(amount)` | Sistema (Token-2022) | Aplica Anti-Whale + Exit Fee, cobra atómico directo al Vault Core |
| `register_market_maker(pk)` | authority + Tridente 3-de-3 | Añade MM a la lista de exentos |
| `revoke_market_maker(pk)` | authority + Tridente 3-de-3 | Quita MM de la lista |

### 3.4 Errores nuevos

```rust
#[msg("MM registry ya existe / no existe para esa pubkey")]
MMRegistryInvalid,
#[msg("Pool liquidez cero o no disponible")]
PoolLiquidityMissing,
#[msg("Transferencia rechazada: sender es la Pubkey::default()")]
InvalidSender,
```

### 3.5 Eventos nuevos

```rust
#[event]
pub struct AntiWhaleTriggered {
    pub sender: Pubkey,
    pub amount_luka: u64,
    pub tx_pct_pool_bps: u64,
    pub excedente_luka: u64,
    pub fee_luka: u64,
}

#[event]
pub struct ExitFeeTriggered {
    pub sender: Pubkey,
    pub amount_luka: u64,
    pub luka_price: u64,
    pub ema30: u64,
    pub fee_luka: u64,
    pub stage: u8,
}

#[event]
pub struct ShieldFeeCollected {
    pub sender: Pubkey,
    pub fee_luka: u64,
    pub fee_usdc_to_vault: u64,
    pub source: u8,  // 0=Anti-Whale, 1=Exit Fee, 2=ambos
}

#[event]
pub struct MarketMakerRegistered { pub mm: Pubkey, pub ts: i64 }
#[event]
pub struct MarketMakerRevoked    { pub mm: Pubkey, pub ts: i64 }
```

---

## 4. Pruebas

### 4.1 Happy path

- **Compra al pool** (from pool to user): sin fee (ADR-012 C10 — solo ventas).
- **Venta de 0.5% del pool** por un usuario random: sin Anti-Whale, sin Exit Fee.
- **Venta de 3% del pool** por usuario random: Anti-Whale tier 2 (6% sobre
  excedente 2%) → fee retenido atómicamente, sender recibe `amount - fee`.
- **Venta de 7% del pool** por usuario random: Anti-Whale tier 3 (10% sobre
  excedente 6%) → fee = 0.6% del pool en LUKA, cobrado en la misma tx.
- **Venta en Génesis con precio en pánico** (`p < 0.7×EMA30` AND `volumen>0.3% supply/h`):
  Exit Fee 5% sobre monto total → cobro atómico, sender recibe 95%.
- **Venta con nivel Titán (Aura ≥ 25K)**: exento, sin fee.
- **Anti-Whale + Exit Fee simultáneos**: ambos fees se suman y se cobran en una sola tx.

### 4.2 Edge cases

- **Sender es MM registrado activo**: exento, `is_registered_mm` retorna true.
- **Sender era MM y fue revocado**: `is_active = false` → NO exento.
- **Swap interno Motor D (CPI desde el programa)**: exento.
- **Sender = Pubkey::default()**: `InvalidSender`.
- **Pool liquidez = 0** (deshabilitado momentáneamente): `PoolLiquidityMissing`,
  la transferencia falla — mejor pausar que penalizar mal.
- **Anti-Whale + Exit Fee simultáneos**: se suman ambos fees y se cobran
  atómicamente. El sender recibe `amount - fee_aw - fee_exit`.

### 4.3 Invariantes nuevos

- **I20:** los $LUKA cobrados atómicamente por el `transfer_hook` se swapean a USDC
  vía Jupiter CPI → entran a `usdc_res_amount` del Vault Core. 100% al Vault, nada a
  O&M ni Staking (ADR-012 explícito). Cobro garantizado en la misma tx, sin deuda pendiente.
- **I21:** para toda transferencia identificada como COMPRA (from pool), el hook no
  cobra fee. Anti-Whale aplica solo a ventas al pool de liquidez (donde la métrica de
  % del pool tiene sentido). Las transferencias P2P no disparan Anti-Whale (no hay pool
  contra el cual medir el umbral); el Exit Fee dual ya cubre el escenario de pánico en
  P2P si las condiciones precio+volumen se cumplen.
- **I22:** `sell_pressure_1h_supply_bps` se resetea cuando `now - last_reset_ts >= 3600`.

### 4.4 Prueba de simulación

Añadir a `simulations/trajectory.py` un escenario **"Ballena venta 10% pool"** y
verificar:

- Sin Anti-Whale: espiral aumenta significativamente (vs baseline BASE).
- Con Anti-Whale: espiral se mantiene ~0% (el fee del 10% sobre 9% de excedente
  al Vault cierra el loop deflacionario y estabiliza).

Es la validación empírica de que el KASH Shield hace lo que dice.

---

## 5. Fuera de alcance

- **Confidential Transfer** de Token-2022: no; queremos observabilidad.
- **Rebajas de fee por comportamiento** (loyalty program): fuera de v1; el DAO
  en Etapa 4 puede añadirlo.
- **Fee sobre transferencias P2P estilo transfer-fee estático**: no. El fee es
  dinámico (Anti-Whale + Exit Fee) o cero.
- **Listing en CEXs**: fuera del contrato — coordinación operativa pre-TGE.

---

## 6. Done cuando

- [ ] Constantes nuevas en `constants.rs`.
- [ ] `MMRegistry` PDA definida.
- [ ] `transfer_hook`, `register/revoke_market_maker` implementadas.
- [ ] Estado `sell_pressure_1h_supply_bps` + rolling window de 1h funcionan.
- [ ] `is_exempt()` en el orden fail-fast correcto.
- [ ] 3 errores + 5 eventos nuevos definidos.
- [ ] Programa deployado en devnet con **mint Token-2022 dedicado** (nuevo,
      aparte del clásico existente).
- [ ] Tests: happy path × 7 + edge cases × 6 pasan.
- [ ] Invariantes I20-I22 pasan.
- [ ] Simulación: escenario "Ballena 10% pool" con Anti-Whale mantiene 0% espiral.
- [ ] Verificación de compatibilidad: Jupiter route funciona · Meteora LP funciona.
- [ ] BITÁCORA + ADR-015 apéndice de implementación actualizados.
