# Spec 07 — Cuentas y Custodia del Protocolo LUKASH (transversal Milestone 2)

> Milestone 2 · Referencia: ADR-015 · Prioridad: **fundamental**
> Este documento es transversal — define TODAS las cuentas del protocolo en un
> solo lugar. Cada spec (07-a a 07-f) referencia aquí en vez de repetir el mapa.

---

## 1. Filosofía de custodia — leer primero

**Principio fundacional:** en LUKASH **NO existen "billeteras temporales" ni
custodias humanas** para los fondos del protocolo. Todas las cuentas críticas
(Vault Core, Vault Sociedad, Staking Pool, R_op, Mint Authority, la propia "cola
de quema") son **PDAs** — Program Derived Addresses cuya autoridad es el propio
programa `lukash_protocol`, no una persona con clave privada.

### Qué es un PDA (en simple)

Una cuenta cuya dirección se calcula determinísticamente a partir del `program_id`
+ semillas (`seeds`). **No tiene clave privada.** La única forma de mover fondos
de un PDA es que el programa mismo lo firme (`invoke_signed`). Un humano no puede
firmar por una PDA aunque quiera — es criptográficamente imposible.

### Consecuencia práctica

- **Auditar el código del programa = auditar todas las billeteras al mismo tiempo.**
  Halborn/OtterSec/Sec3/etc auditan `lib.rs`, no procedimientos operativos de
  wallets separadas.
- **El "riesgo humano"** (fuga, extorsión, pérdida de clave) **no aplica a las PDAs.**
  Solo aplica a las 2 wallets multisig reales (§4).
- **Todo es verificable on-chain**: cualquiera puede leer los PDAs y confirmar
  balances, autoridad y estado sin permiso.

### Las únicas 2 wallets con custodia humana

Solo dos cuentas tienen humanos firmando: la **Wallet O&M** (sueldos e infra del
equipo) y el **Tridente Multisig** (3-de-3 para operaciones críticas de emergencia).
Ambas son multisig — nunca clave individual. Se documentan en §4.

---

## 2. Mapa completo de cuentas del protocolo

### 2.1 Tabla resumen

| # | Cuenta | Tipo | Autoridad | Milestone | Puede entrar | Puede salir |
| ---: | --- | --- | --- | :---: | --- | --- |
| 1 | **ProtocolConfig PDA** | PDA `[b"config"]` | Programa | 1 | — (parámetros) | — (Timelock 48h para cambios) |
| 2 | **ProtocolState PDA** | PDA `[b"state"]` | Programa | 1 | Actualizaciones vía instrucciones | No aplica (es estado, no fondos) |
| 3 | **Vault Core — cBTC ATA** | PDA `[b"vault_core", "cbtc"]` | Programa | 2 | Jupiter CPI (compra) | Solo Capa 3 emergencia (Tridente 3-de-3) — Milestone 3 |
| 4 | **Vault Core — SOL** | PDA `[b"vault_core", "sol"]` | Programa | 2 | Jupiter CPI | No sale (por diseño) |
| 5 | **Vault Core — LST ATA** | PDA `[b"vault_core", "lst"]` | Programa | 2 | Jupiter CPI + rebase de yield | No sale |
| 6 | **Vault Core — USDC Reserva ATA** | PDA `[b"vault_core", "usdc_res"]` | Programa | 2 | Jupiter CPI + `receive_insurance_recovery` | No sale |
| 7 | **Vault Core — USDC Lending ATA** | PDA `[b"vault_core", "usdc_lend"]` | Programa | 2 | Jupiter CPI + deposit Kamino/Marginfi | Yield rebase (interno) |
| 8 | **Vault Sociedad ATA** (USDC) | PDA `[b"vault_sociedad"]` | Programa | 2 | Split 30% del Asset Layer | Distribución a socios desde mes 13 (Etapa 2B) — Milestone 3 |
| 9 | **Wallet O&M** | **Multisig humano** | Equipo (2-de-N) | 2 | Split 15% de fees | Salidas operativas (multisig) |
| 10 | **Staking Pool ATA** | PDA `[b"staking_pool"]` | Programa | 2 | Split 15% de fees | Claims de stakers (usuarios) |
| 11 | **R_op ATA** (USDC) | PDA `[b"r_op"]` | Programa | 2 | 30% del yield del Vault | Recarga Forzada UCR (Milestone 3) |
| 12 | **LP Meteora Pool** | Externa (Meteora) | Meteora program | 2 | Recirculación B2 + LP Fundador | Trading (mercado) |
| 13 | **LP Fundador ATA** | Wallet Sebastián con lock 365d on-chain | Sebastián + lock | 2 | Aporte inicial pre-TGE | Solo tras 365d |
| 14 | **Mint del token $LUKA** | Token-2022 mint | Autoridad = PDA `[b"mint_authority"]` | 2 mainnet (devnet: SPL clásico existente) | `mint_to` inicial | `burn` (drenaje cola + quema diaria) |
| 15 | **MMRegistry PDA** (por MM) | PDA `[b"mm_registry", <pubkey_mm>]` | Programa | 2 | `register_market_maker` (Tridente) | `revoke_market_maker` (Tridente) |
| 16 | **WhaleDebt PDA** (por sender) | PDA `[b"whale_debt", <pubkey_sender>]` | Programa | 2 | `transfer_hook` acumula | `collect_whale_debt` cobra al Vault Core |
| 17 | **Tridente Multisig** | **Multisig 3-de-3** | 3 humanos (pendientes pre-TGE) | 2 (inactivo) → 3 mainnet (activo) | — | Firma para: activar Capa 3, cancelar CB, modificar K_min, registrar MMs, `receive_insurance_recovery` |

### 2.2 La "cola de quema" NO es una cuenta

Merece aclaración explícita porque es contraintuitivo: `deferred_burn_queue` es un
**campo del ProtocolState** (`u64` en USD 6-dec) — solo un contador. No hay una
"wallet de cola" con tokens dentro. Cuando llega el momento del drenaje semanal,
el contrato usa el flujo de fees futuro para comprar $LUKA y quemarlo (`burn` CPI
al mint) — no hay tokens "guardados" esperando en ningún lado.

**Ventaja:** cero superficie de ataque (no hay honeypot de "cola"), cero riesgo de
clave perdida, verificable públicamente leyendo el estado.

### 2.3 Cuentas Vault Core valorizadas vs contables

- **Balances por bucket** (`cbtc_amount`, `sol_amount`, `lst_amount`,
  `usdc_res_amount`, `usdc_lend_amount`) en el ProtocolState = cantidad real de
  cada activo (unidades nativas: satoshis, lamports, micro-USDC).
- **Cada balance corresponde a una ATA del Vault Core** (cuentas #3-#7 arriba)
  donde viven los tokens reales.
- **Fuente única de verdad:** las ATAs. El estado del programa debe reconciliar
  contra ellas via CPI de lectura antes de cada operación crítica (invariante I23).

---

## 3. Ciclo de vida de cada cuenta

### 3.1 Creación (pre-TGE / initialize)

En `initialize(ctx)`:
- Se crean **PDAs 1, 2, 8, 10, 11, 14** (config, state, sociedad, staking, r_op,
  mint_authority).
- **Cuentas Vault Core (#3-#7)** se crean como ATAs del PDA autoridad la primera
  vez que reciben tokens (patrón lazy — reduce cost inicial). Verificado on-chain.
- **Cuenta LP Fundador (#13):** creada por Sebastián con su wallet, seed inicial de LP.

En `activate_tridente(pk1, pk2, pk3)` (post-initialize, pre-Etapa 2):
- Registra las 3 pubkeys del multisig. Antes: `Pubkey::default()`.

### 3.2 Operación normal (Milestone 2)

- **Entrada de fees** (`process_fee`): CPIs a Jupiter mueven tokens del pool al
  Vault Core, split Sociedad va a #8, O&M a #9, Staking a #10.
- **Quema**: `burn_luka_cpi` con `mint_authority` como firmante — reduce supply del
  mint sin mover tokens de wallets.
- **Yield**: LST y USDC Lending crecen por rebase de terceros (Jito, Kamino) —
  automático, no requiere firma.
- **Reconciliación** (`refresh_vault_valuation`): permissionless, cualquiera lee las
  ATAs y actualiza el `k_market_usd_snapshot` del state.

### 3.3 Emergencia (Milestone 2)

- **Circuit Breaker del Vault**: pausa 24h → `process_fee`, `switch_motor_b`,
  `execute_deferred_burn` bloqueados. Cancelable temprano por Tridente 3-de-3.
- **Seguro Anti-Exploit**: la aseguradora deposita USDC al Vault Core USDC Reserva
  (#6) via `receive_insurance_recovery` — flujo de entrada, requiere Tridente.
- **Capa 3 cBTC (Milestone 3)**: única salida contemplada del Vault Core; requiere
  Tridente 3-de-3 y procedimiento operativo Sanctum. Milestone 2 deja stub protegido.

---

## 4. Las 2 wallets con custodia humana — procedimientos operativos

### 4.1 Wallet O&M (Multisig del equipo)

**Función:** recibe el 15% de todos los fees (SOL / USDC según motor). Paga:
- Sueldos del equipo (fundador + devs + growth + comunidad).
- Infraestructura (RPCs, oráculos premium, dashboards, hosting).
- Auditorías externas.
- KOLs paid en cash (raro; la mayoría vestean tokens — ADR-011).
- Legal & compliance.

**Estructura recomendada:** multisig 2-de-3 con **Squads Protocol** (el estándar
Solana para multisig, auditado). Firmantes propuestos:
- Sebastián (fundador).
- Un dev senior del equipo (rotable con vesting cliff).
- Una tercera parte de confianza (contador, abogado, o board advisor).

**Regla operativa:** ninguna salida > $10K sin explicación en un ADR de gasto
(o un log público mensual). Transparencia como práctica, no obligación legal
(hasta Etapa 4 donde el DAO impone reglas).

**Estado actual:** wallet única de Sebastián (single-sig). **Bloqueante pre-TGE:**
crear el multisig 2-de-3 antes de que O&M reciba fees reales.

### 4.2 Tridente Multisig (3-de-3)

**Función:** firma para operaciones críticas del Vault:
1. Activar Capa 3 (cBTC Reserva Profunda) en emergencia — Milestone 3.
2. Cancelar Circuit Breaker LP antes de las 24h automáticas.
3. Modificar K_min (parámetro del protocolo, requiere Timelock 48h).
4. Registrar / revocar Market Makers (`register_market_maker` / `revoke_market_maker`).
5. Autorizar `receive_insurance_recovery` (evita depósitos falsos que gasten la
   ventana anual de 12 meses).

**Estructura obligada por el contrato:** 3-de-3 exacto. Cada firmante distinto.
Ninguno puede ser la authority del programa (separación de roles enforced).

**Estado actual:** **inactivo** (`tridente_activated = false`). Toda la lógica del
3-de-3 está construida, testeada y auditable — pero mientras esté inactivo, las 5
operaciones críticas listadas arriba **fallan con `TridenteNotActivated`**.

**Candado estructural:** el contrato **rechaza el paso a Etapa 2** si el Tridente
no está activado. Es decir: **imposible ir a mainnet post-TGE sin haber activado
el Tridente**, incluso por olvido.

**Estado en devnet Milestone 1-2:** Sebastián es la única authority; opera en
Etapa 1 (Génesis); las 5 operaciones críticas no son necesarias en Etapa 1.
Bien.

**⚠️ BLOQUEANTE PRE-TGE (importante):** antes del TGE mainnet:
1. Identificar los 3 firmantes reales del Tridente (2 pendientes — Sebastián solo).
2. Crear los 3 wallets (idealmente con hardware wallets — Ledger).
3. Publicar sus pubkeys en un ADR previo a la activación.
4. Ejecutar `activate_tridente(pk1, pk2, pk3)` — **irreversible**.
5. Confirmar en el explorador que las 3 pubkeys están registradas.
6. Solo entonces encolar el cambio a Etapa 2.

Este checklist pre-TGE va en la spec 07-INDICE §10 y se recuerda en cada session-close.

---

## 5. Invariantes de custodia (validables en `simulations/suite.py`)

Añadir a SIM 0:

- **I23 (reconciliación):** `state.cbtc_amount == balance(cuenta #3)`,
  `state.sol_amount == balance(cuenta #4)`, y así con los 5 buckets. El estado
  del programa debe coincidir con las ATAs on-chain. Si divergen → bug crítico.

- **I24 (autoridad de PDA):** las cuentas #3-#8, #10, #11 tienen autoridad ==
  PDA derivado del programa. Ninguna tiene autoridad humana.

- **I25 (mint authority):** la autoridad del mint del token = PDA
  `[b"mint_authority"]`. Ningún humano puede llamar `mint_to` o `burn`
  directamente al mint sin pasar por el programa.

- **I26 (integridad del multisig O&M):** no aplica on-chain (es multisig externo),
  pero el ADR de la wallet O&M debe estar publicado y sus firmantes verificables.

- **I27 (Tridente monotónico):** una vez activado, siempre activado. Ya cubierto
  como I16 en spec 07-c.

---

## 6. Diagrama de flujo (Milestone 2 completo)

```
                    ┌─────────────┐
                    │   Usuario   │
                    └──────┬──────┘
                           │ paga fee
                           ▼
              ┌───────────────────────────┐
              │  process_fee (35/35/15/15)│
              └───────────┬───────────────┘
                          │
       ┌──────────────────┼──────────────────┬───────────────┐
       │ 35% Asset Layer  │ 35% LP/Quema     │ 15% O&M       │ 15% Staking
       ▼                  ▼                  ▼               ▼
  ┌─────────┐      B0: burn CPI mint    ┌─────────┐    ┌──────────┐
  │Jupiter  │      B2: LP Meteora       │Wallet   │    │Staking   │
  │CPI x 5  │      (07-a)               │O&M      │    │Pool PDA  │
  └────┬────┘                           │multisig │    │(#10)     │
       │                                │2-de-3   │    └──────────┘
       ├──70% Core────┐                 │(#9)     │
       │              │                 └─────────┘
       │      ┌───────┴──────┐
       │      │ Vault Core   │
       │      │ (#3-#7 ATAs) │
       │      │ [PDAs, no    │
       │      │  claves]     │
       │      └──────────────┘
       │
       └──30% Sociedad──┐
                        ▼
                 ┌──────────────┐
                 │Vault Sociedad│
                 │(#8 PDA)      │
                 │KASH Lock     │
                 │hasta mes 13  │
                 └──────────────┘

  Emergencia (Tridente 3-de-3):
    → cancelar Circuit Breaker
    → activar Capa 3 (cBTC) — Milestone 3
    → registrar MM (#15)
    → receive_insurance_recovery (aseguradora → #6)
    → modificar K_min (+ Timelock 48h)

  Yield del Vault:
    LST rebase → suma a #5
    USDC Lending → suma a #7
    30% → R_op (#11, idle)
```

---

## 7. Checklist pre-TGE (bloqueante mainnet)

- [ ] **Mint Token-2022** creado con Transfer Hook apuntando a `lukash_protocol`.
- [ ] **Metadata** aplicada (nombre, logo, URI — como devnet actual con Metaplex nativo Token-2022).
- [ ] **PDAs 1-8, 10, 11, 14, 17** verificadas en el explorer post-`initialize`.
- [ ] **Wallet O&M** creada como multisig 2-de-3 con Squads Protocol; ADR publicado con firmantes.
- [ ] **Tridente Multisig**: 3 wallets identificadas (idealmente hardware wallets), pubkeys publicadas en ADR, `activate_tridente(pk1, pk2, pk3)` ejecutada, verificación en explorer.
- [ ] **LP Fundador**: aporte inicial en Meteora con lock 365d on-chain verificable.
- [ ] **Auditoría externa** completada (vía subsidio Colosseum/Areta/Superteam).
- [ ] **Simulación con contrato deployado** en devnet 2+ semanas sin bugs.
- [ ] Solo entonces: encolar `execute_admin_change(kind=STAGE, value=2)` — el candado estructural verifica Tridente activo y lo permite.

---

## 8. Fuera de alcance de esta spec

- **Procedimiento detallado de rotación de firmantes del O&M**: se define en ADR
  específico cuando el equipo crezca.
- **Wallet de tesorería del DAO en Etapa 4**: fuera de Milestone 2 completamente.
- **Wallets de KOLs / MMs específicos**: cada uno tiene su propio setup; la
  responsabilidad del protocolo es exponer los slots (LP Comprometido lock,
  MMRegistry) — no gestionar sus wallets.
- **Bridge de fondos entre Solana y otras chains**: por diseño, LUKASH es 100%
  Solana-nativo (ADR-P01). No hay bridge.

---

## 9. Recordatorio para futuras sesiones

**Antes del TGE mainnet, Sebastián debe:**

1. Definir los 3 firmantes del Tridente Multisig (hoy pendientes).
2. Definir los 2-3 firmantes del multisig O&M (hoy single-sig).
3. Publicar ADR con las pubkeys de ambos multisigs.
4. Ejecutar `activate_tridente` post-deploy mainnet.

**Este ítem se propagará automáticamente al todo y al session-close hasta que se
resuelva.** El contrato tiene el candado estructural que impide olvidarlo, pero
el trabajo de identificar y crear los wallets es humano.

---

*Spec transversal de cuentas y custodia. Referenciada por 07-a, 07-c, 07-e, 07-f.
Base: ADR-015. Base fundacional: cero billeteras humanas para fondos del protocolo;
todo es PDA excepto O&M multisig y Tridente Multisig, ambas 100% documentadas aquí.*
