# Spec 07-c — Jaguar Shield del Vault: Tridente inactivo + Circuit Breaker + Seguro Anti-Exploit

> Milestone 2 · Referencia: Protocolo v4.3 §9 (ampliado) + ADR-015 · Prioridad: **alta**
> Dependencias upstream: 07-a (token accounting real).
> Habilita: Etapa 2 (App launch) via candado estructural — sin esta spec no puedes ir a mainnet.

---

## 1. Problema (qué hay hoy)

`contracts/playground/lib.rs` tiene ya un patrón de `AdminOnly` + `queue_admin_change`
+ `execute_admin_change` + Timelock 48h — la infraestructura de gobernanza básica.
**Faltan tres piezas del Jaguar Shield del Vault**, todas confirmadas por ADR-015:

1. **Tridente Multisig 3-de-3** cableado pero **INACTIVO por defecto**, con
   activación one-way y candado estructural que bloquea el paso a Etapa 2 si no
   se activó. (Aprobado por Sebastián 2026-08-21.)
2. **Circuit Breaker del Vault** — pausa de 24h ante variación negativa >10% en 1h.
3. **`receive_insurance_recovery`** — instrucción de entrada al Vault cuando la
   aseguradora reembolsa un exploit (nunca hay salidas hacia la aseguradora —
   es un flujo unidireccional hacia adentro).

**Principio general (v1.0 ratificado 2026-08-21):** el Vault **no se toca** hasta
Etapa 4 (DAO). Esta spec no crea ninguna nueva ruta de salida — solo blinda las que
ya existen y prepara la entrada del seguro.

## 2. Diseño

### 2.1 Tridente Multisig — patrón "inactivo por defecto + candado estructural"

**Estado nuevo en `ProtocolConfig`:**

```rust
pub tridente_activated:   bool,     // false al initialize
pub tridente_signer_1:    Pubkey,   // Pubkey::default() hasta activación
pub tridente_signer_2:    Pubkey,
pub tridente_signer_3:    Pubkey,
pub tridente_activated_ts: i64,     // 0 hasta activación
```

**Nueva instrucción one-way `activate_tridente`:**

```rust
pub fn activate_tridente(
    ctx: Context<AdminOnly>,
    pk1: Pubkey,
    pk2: Pubkey,
    pk3: Pubkey,
) -> Result<()> {
    let cfg = &mut ctx.accounts.config;
    require!(!cfg.tridente_activated, LukashError::TridenteAlreadyActivated);
    // No default keys
    require!(pk1 != Pubkey::default(), LukashError::InvalidTridenteSigner);
    require!(pk2 != Pubkey::default(), LukashError::InvalidTridenteSigner);
    require!(pk3 != Pubkey::default(), LukashError::InvalidTridenteSigner);
    // Todos distintos entre sí
    require!(pk1 != pk2 && pk2 != pk3 && pk1 != pk3, LukashError::TridenteSignersMustBeDistinct);
    // Ninguno coincide con la authority actual (separación de roles)
    require!(pk1 != cfg.authority && pk2 != cfg.authority && pk3 != cfg.authority,
        LukashError::TridenteSignerCannotBeAuthority);
    cfg.tridente_signer_1 = pk1;
    cfg.tridente_signer_2 = pk2;
    cfg.tridente_signer_3 = pk3;
    cfg.tridente_activated = true;
    cfg.tridente_activated_ts = Clock::get()?.unix_timestamp;
    emit!(TridenteActivated { pk1, pk2, pk3, ts: cfg.tridente_activated_ts });
    Ok(())
}
```

**Guard `assert_tridente_signed(ctx)` para las 3 operaciones críticas** (activar
Capa 3 cBTC, cancelar Circuit Breaker LP, modificar K_min):

```rust
fn assert_tridente_signed(cfg: &ProtocolConfig, signers: &[AccountInfo]) -> Result<()> {
    require!(cfg.tridente_activated, LukashError::TridenteNotActivated);
    // Buscar las 3 pubkeys registradas entre los signers de la tx
    let s1 = signers.iter().any(|a| a.key == &cfg.tridente_signer_1 && a.is_signer);
    let s2 = signers.iter().any(|a| a.key == &cfg.tridente_signer_2 && a.is_signer);
    let s3 = signers.iter().any(|a| a.key == &cfg.tridente_signer_3 && a.is_signer);
    require!(s1 && s2 && s3, LukashError::TridenteSignaturesIncomplete);
    Ok(())
}
```

**Candado estructural — bloqueo del paso a Etapa 2:**

En `execute_admin_change` cuando `kind == STAGE` y `value == 2`:

```rust
2 if config.pending_kind == 1 && config.pending_value == 2 => {
    require!(config.tridente_activated, LukashError::TridenteRequiredForStage2);
    // ... resto de la ejecución
}
```

**Efecto:** en devnet Milestone 1-2, opero en Etapa 1 con Tridente inactivo (yo
soy la authority única, es válido). Cuando intento avanzar a Etapa 2 (App launch)
en cualquier red, el contrato exige que ya se activó el Tridente. Sin escapatoria.

### 2.2 Circuit Breaker del Vault

**Estado nuevo en `ProtocolState`:**

```rust
pub cb_active_until_ts:  i64,     // 0 si no pausado; timestamp fin de pausa si activo
pub cb_last_snapshot_usd: u64,   // K_market snapshot hace 1h para detectar caída
pub cb_last_snapshot_ts:  i64,
```

En `refresh_vault_valuation` (07-a), después de calcular `k_market_new`:

```rust
let now = Clock::get()?.unix_timestamp;
// Ventana rodante de 1h
if state.cb_last_snapshot_ts == 0 || now - state.cb_last_snapshot_ts >= 3600 {
    // Verificar caída >10% en la última hora
    if state.cb_last_snapshot_usd > 0 {
        let prev = state.cb_last_snapshot_usd;
        // caída = (prev - now) / prev >= 10%
        if k_market_new < prev.saturating_sub(prev / 10) {
            // Pausa 24h
            state.cb_active_until_ts = now + 24 * 3600;
            emit!(CircuitBreakerTriggered {
                prev_usd: prev,
                current_usd: k_market_new,
                paused_until: state.cb_active_until_ts,
            });
        }
    }
    // Actualizar snapshot
    state.cb_last_snapshot_usd = k_market_new;
    state.cb_last_snapshot_ts = now;
}
```

**Efecto de la pausa** — en `process_fee`, `switch_motor_b`, `execute_deferred_burn`:

```rust
let now = Clock::get()?.unix_timestamp;
require!(now >= state.cb_active_until_ts, LukashError::CircuitBreakerActive);
```

**Cancelación temprana del Circuit Breaker:** requiere Tridente 3-de-3. Nueva
instrucción `cancel_circuit_breaker(ctx)` que llama `assert_tridente_signed` y
resetea `cb_active_until_ts = 0`. Emite evento `CircuitBreakerCancelled`.

Esto conecta con el ítem del ADR-015: "Tridente 3-de-3 para: activar Capa 3,
**cancelar Circuit Breaker LP**, modificar K_min". La primera y tercera son
existentes; esta es la segunda.

### 2.3 Seguro Anti-Exploit — instrucción `receive_insurance_recovery`

**Solo flujo de entrada.** La aseguradora deposita USDC (o el activo elegido por
la póliza) al Vault; el contrato lo registra como refill de emergencia sin
distorsionar la contabilidad de fees.

**Condiciones (del ADR-015):**

- Cobertura hasta 5% del valor total del Vault por evento.
- Máximo 1 evento cada 12 meses.
- Activo desde **Etapa 2B** (`motor_b_state == MOTOR_B_B2`).
- En **Etapa 4 (DAO)** el DAO reemplaza estas reglas.

**Estado nuevo:**

```rust
pub last_insurance_recovery_ts: i64,    // 0 si nunca
pub insurance_recoveries_total_usd: u64,
```

**Instrucción:**

```rust
pub fn receive_insurance_recovery(
    ctx: Context<ReceiveInsuranceRecovery>,
    amount_usdc: u64,
) -> Result<()> {
    let cfg = &ctx.accounts.config;
    let state = &mut ctx.accounts.state;
    let now = Clock::get()?.unix_timestamp;

    // Firma del Tridente (protege contra depósitos falsos que triggereen la ventana anual)
    assert_tridente_signed(cfg, ctx.remaining_accounts)?;

    // Activo solo desde Etapa 2B
    require!(state.motor_b_state == MOTOR_B_B2, LukashError::InsuranceNotYetActive);

    // Ventana de 12 meses
    let elapsed = now.saturating_sub(state.last_insurance_recovery_ts);
    require!(
        state.last_insurance_recovery_ts == 0 || elapsed >= 365 * 86_400,
        LukashError::InsuranceCooldown
    );

    // Cap 5% del valor actual del Vault
    let max_recovery = state.k_market_usd_snapshot / 20;  // 5%
    require!(amount_usdc <= max_recovery, LukashError::InsuranceExceedsCap);

    // CPI: transferir USDC desde la wallet de la aseguradora al ATA del Vault
    // (aquí la interfaz — cuenta concreta se resuelve en integración)
    transfer_usdc_to_vault_cpi(ctx, amount_usdc)?;

    // Registrar como buffer USDC del Vault (Capa 1)
    state.usdc_res_amount = state.usdc_res_amount
        .checked_add(amount_usdc).ok_or(LukashError::MathOverflow)?;
    state.last_insurance_recovery_ts = now;
    state.insurance_recoveries_total_usd = state.insurance_recoveries_total_usd
        .checked_add(amount_usdc).ok_or(LukashError::MathOverflow)?;

    emit!(InsuranceRecoveryReceived {
        amount_usdc,
        cap_at_event: max_recovery,
        ts: now,
    });
    Ok(())
}
```

**Nota:** en Etapa 4 (DAO) el contrato pasa el control de estas condiciones a la
gobernanza — se añade un feature flag `insurance_rules_by_dao` que el DAO puede
activar (Milestone 3).

### 2.4 El Vault NO se toca (invariante)

Esta spec **no crea ninguna instrucción que decremente balances del Vault Core.**
Las únicas mutaciones posibles siguen siendo:

- **Aumentar** (`process_fee` → CPI Jupiter → suma a `*_amount`).
- **Aumentar por yield** (`yield_rebase` — parte de 07-a).
- **Aumentar por seguro** (`receive_insurance_recovery` — esta spec, USDC-res only).

La única salida contemplada del diseño es **Capa 3 (cBTC Reserva Profunda) en emergencia**,
gated por Tridente 3-de-3. Esa instrucción (`activate_layer_3_emergency`) queda
diseñada aquí como **stub protegido** que emite un evento pero **no ejecuta la
transferencia real** en Milestone 2 — la ejecución real requiere infra Sanctum
+ Meteora + procedimiento operativo detallado que corresponde a Milestone 3 (o
directamente Etapa 4 DAO).

---

## 3. Contrato — interfaces

### 3.1 Errores nuevos

```rust
#[msg("Tridente Multisig no activado. Llamar activate_tridente(pk1, pk2, pk3) primero")]
TridenteNotActivated,
#[msg("Tridente ya activado (one-way, irreversible)")]
TridenteAlreadyActivated,
#[msg("Firmante del Tridente inválido (pubkey cero)")]
InvalidTridenteSigner,
#[msg("Los 3 firmantes del Tridente deben ser distintos entre sí")]
TridenteSignersMustBeDistinct,
#[msg("Un firmante del Tridente no puede ser también la authority")]
TridenteSignerCannotBeAuthority,
#[msg("Faltan firmas del Tridente en la transacción (requeridas 3-de-3)")]
TridenteSignaturesIncomplete,
#[msg("Etapa 2 (App launch) requiere el Tridente activado — activate_tridente primero")]
TridenteRequiredForStage2,
#[msg("Circuit Breaker del Vault activo — protocolo en pausa por 24h")]
CircuitBreakerActive,
#[msg("Seguro Anti-Exploit activo solo desde Etapa 2B (Motor B2)")]
InsuranceNotYetActive,
#[msg("Cooldown del Seguro: máximo 1 evento cada 12 meses")]
InsuranceCooldown,
#[msg("Monto excede el cap del 5% del Vault en este evento")]
InsuranceExceedsCap,
```

### 3.2 Eventos nuevos

```rust
#[event]
pub struct TridenteActivated {
    pub pk1: Pubkey,
    pub pk2: Pubkey,
    pub pk3: Pubkey,
    pub ts: i64,
}

#[event]
pub struct CircuitBreakerTriggered {
    pub prev_usd: u64,
    pub current_usd: u64,
    pub paused_until: i64,
}

#[event]
pub struct CircuitBreakerCancelled {
    pub cancelled_at: i64,
    pub was_until: i64,
}

#[event]
pub struct InsuranceRecoveryReceived {
    pub amount_usdc: u64,
    pub cap_at_event: u64,
    pub ts: i64,
}
```

### 3.3 Instrucciones nuevas resumidas

| Instrucción | Firmante | One-way | Cuentas críticas |
| --- | --- | --- | --- |
| `activate_tridente(pk1, pk2, pk3)` | authority | **Sí** | Config |
| `cancel_circuit_breaker` | Tridente 3-de-3 | No | Config + State + 3 signers |
| `receive_insurance_recovery(amount)` | Tridente 3-de-3 | No (con cooldown 12m) | Config + State + USDC ATAs + 3 signers |

### 3.4 Instrucciones modificadas

- `execute_admin_change` (kind=STAGE, value=2): añadir `require!(tridente_activated)`.
- `process_fee`, `switch_motor_b`, `execute_deferred_burn`: añadir chequeo del
  Circuit Breaker (`now >= cb_active_until_ts`).
- `refresh_vault_valuation` (de 07-a): incorporar la lógica de detección de caída
  >10%/1h y disparo del CB.

---

## 4. Pruebas

### 4.1 Happy path

- `activate_tridente(pk_alice, pk_bob, pk_carol)` con authority = Sebastián →
  activado, evento emitido.
- Segundo `activate_tridente` → `TridenteAlreadyActivated`.
- Intento pasar a Etapa 2 sin activar Tridente → `TridenteRequiredForStage2`.
- Después de activar y encolar cambio a Etapa 2 → `execute_admin_change` funciona.
- Circuit Breaker: K cae de $30M a $26M en 1h → CB activa 24h → `process_fee`
  falla con `CircuitBreakerActive`.
- Tridente 3-de-3 firma `cancel_circuit_breaker` → CB desactivado.
- Aseguradora deposita $500K USDC (Vault vale $30M, cap = $1.5M, ok) → `usdc_res_amount`
  sube en 500K micro-USDC × 1e6.

### 4.2 Edge cases

- **`activate_tridente` con pk1 == pk2:** `TridenteSignersMustBeDistinct`.
- **`activate_tridente` con pk1 == authority:** `TridenteSignerCannotBeAuthority`.
- **`activate_tridente` con Pubkey::default():** `InvalidTridenteSigner`.
- **`cancel_circuit_breaker` con solo 2 firmantes del Tridente:** `TridenteSignaturesIncomplete`.
- **`receive_insurance_recovery` en Etapa 1 (B0):** `InsuranceNotYetActive`.
- **Segundo `receive_insurance_recovery` 10 meses después:** `InsuranceCooldown`.
- **`receive_insurance_recovery` con `amount > 5% Vault`:** `InsuranceExceedsCap`.
- **CB dispara pero luego K recupera:** la pausa sigue 24h (por diseño — mejor
  overshooting que undershooting en emergencia); Tridente puede cancelar temprano.

### 4.3 Invariantes nuevos

- **I16:** `tridente_activated` es monotónico (una vez `true`, siempre `true`).
- **I17:** cuando `cb_active_until_ts > now`, ningún `process_fee` puede modificar el estado.
- **I18:** `elapsed >= 365 días` para cada uso consecutivo del seguro.
- **I19:** `receive_insurance_recovery` incrementa `usdc_res_amount` en exactamente
  el monto recibido (nada de otros buckets se toca).

---

## 5. Fuera de alcance

- **Activar Capa 3 cBTC realmente en emergencia** (`activate_layer_3_emergency`): la
  interfaz queda diseñada como stub protegido por Tridente que emite evento;
  la transferencia real es Milestone 3 (requiere procedimiento operativo Sanctum).
- **DAO en Etapa 4** que redefine reglas del seguro: Milestone 3.
- **Contratación real de la aseguradora**: fuera del código; se resuelve pre-TGE
  vía subsidios (Colosseum / Areta / grants). ADR de contratación cuando se cotice.
- **Prima anual on-chain**: la póliza es off-chain (contrato legal con la
  aseguradora). El contrato solo recibe el reembolso si ocurre un evento.

---

## 6. Done cuando

- [ ] Estado nuevo en `ProtocolConfig` (Tridente) y `ProtocolState` (CB + seguro).
- [ ] `activate_tridente` con las 4 validaciones de entrada.
- [ ] `assert_tridente_signed` helper reutilizable.
- [ ] `execute_admin_change` rechaza paso a Etapa 2 sin Tridente activado.
- [ ] `cancel_circuit_breaker` con Tridente 3-de-3.
- [ ] `receive_insurance_recovery` con las 3 validaciones (etapa, cooldown, cap).
- [ ] Todos los `process_fee`, `switch_motor_b`, `execute_deferred_burn` respetan
      `cb_active_until_ts`.
- [ ] `refresh_vault_valuation` dispara CB cuando corresponde.
- [ ] 11 errores + 4 eventos nuevos definidos.
- [ ] Tests: happy path + 8 edge cases pasan.
- [ ] Invariantes I16-I19 pasan en `simulations/suite.py inv`.
- [ ] En devnet: Tridente inactivo, Sebastián como authority única — todas las
      operaciones de Etapa 1 funcionan.
- [ ] Compila y despliega en devnet.
- [ ] BITÁCORA actualizada.
