use anchor_lang::prelude::*;

#[error_code]
pub enum LukashError {
    #[msg("Overflow aritmético")]
    MathOverflow,
    #[msg("La distribución no suma el monto del fee (invariante 35/35/15/15 roto)")]
    DistributionInvariant,
    #[msg("Motor o capa inválidos")]
    InvalidMotorOrLayer,
    #[msg("El motor no está activo en la etapa actual del protocolo")]
    MotorNotActiveInStage,
    #[msg("Etapa del protocolo inválida")]
    InvalidStage,
    #[msg("Solo la autoridad (Tridente Multisig) puede ejecutar esto")]
    Unauthorized,
    #[msg("El protocolo está en pausa (Circuit Breaker)")]
    ProtocolPaused,
    #[msg("No hay un cambio de parámetro encolado")]
    NoPendingChange,
    #[msg("El Timelock de 48h aún no ha transcurrido")]
    TimelockNotElapsed,
    #[msg("El Motor B ya está en estado B2")]
    AlreadyB2,
    #[msg("K(t) aún no alcanza K_min; no se puede conmutar a B2")]
    KminNotReached,
    #[msg("La composición del Vault no suma 100% (10000 bps)")]
    VaultCompositionInvalid,
    #[msg("Monto inválido (cero)")]
    ZeroAmount,
    #[msg("El precio no está normalizado (Throttle no NORMAL/ACELERADO)")]
    ThrottleNotNormalized,
    #[msg("Aún no ha pasado una semana desde la última ejecución de la cola")]
    QueueCooldown,
    #[msg("No hay quemas diferidas en cola")]
    EmptyQueue,
    #[msg("Valor de oráculo inválido (precio o EMA30 en cero)")]
    InvalidOracleValue,
    #[msg("Tipo de cambio de parámetro inválido")]
    InvalidChangeKind,
    #[msg("Autoridad inválida (no puede ser la dirección por defecto)")]
    InvalidAuthorityPubkey,
    #[msg("Divisa inválida (debe ser 0=LUKA o 1=SOL/USDC)")]
    InvalidCurrency,
}
