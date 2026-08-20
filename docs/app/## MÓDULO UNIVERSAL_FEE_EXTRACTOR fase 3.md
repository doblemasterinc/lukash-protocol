## MÓDULO: UNIVERSAL_FEE_EXTRACTOR
1. **Monitor:** Detectar cualquier instrucción de transferencia SPL (SOL, USDC, cBTC).
2. **Execution:** - Calcular el % de fee según `Protocol_Stage`.
   - Ejecutar Swap de la porción del fee a $LUKA.
   - Procesar $LUKA resultante según el `Distribution_Contract` (Burn/Vault/LP/Staking).

## MÓDULO: REWARD_DISTRIBUTOR (GUERRILLA)
1. **Source:** Los fondos para incentivos provienen exclusivamente de `Community_Wallet` o `Marketing_Wallet`.
2. **Condition:** El sistema no debe permitir transacciones de salida desde `Vault_KASH_Core` hacia usuarios.

## MÓDULO: JUNGLE_ARENA_CORE
1. **Energy_Store:** Venta de créditos de energía por $LUKA (Aplicar split 30/30/40).
2. **cNFT_Updater:** Escuchar eventos de la App para actualizar el nivel del Jaguar en el Merkle Tree de Solana.

## MÓDULO: ISO_MEMO_INJECTOR
1. **Automate:** Adjuntar string XML ISO 20022 en el campo `memo` de cada transacción procesada por la App LUKASH, sin importar el activo.