# RUNBOOK — Disparar deploy + token $LUKA en devnet (Solana Playground)

> Todo listo para "copiar-pegar" el día que haya SOL de prueba. El ÚNICO bloqueo actual es el
> faucet de devnet (rate-limit/caído). Nada de esto requiere instalar nada — todo en https://beta.solpg.io
>
> Estado 2026-08-20: contrato COMPILA ("Build successful"). Falta SOL para desplegar. Ver §0.

---

## §0 — Conseguir SOL de prueba (el bloqueo actual)
Necesitas **~1.5 SOL** en la wallet de Playground. Orden de intento:
1. **CLI, poco a poco:** `solana airdrop 1` (reintentar cada ~30s; probar `0.5`).
2. **Cambiar IP** (hotspot del celular / VPN) y reintentar el CLI. El límite es por IP.
3. **RPC propio (Helius):** cuenta gratis en https://www.helius.dev → copiar RPC devnet
   (`https://devnet.helius-rpc.com/?api-key=xxxx`) → en Playground, clic en el endpoint (junto a la
   wallet) → **Custom** → pegar URL → reintentar `solana airdrop 1`. Esto además evita el bug
   "body stream already read" del RPC por defecto.
4. **Faucets web:** https://faucet.solana.com (pide GitHub con antigüedad) · https://solfaucet.com
5. **Último recurso:** esperar unas horas — los límites de devnet resetean solos.

Verificar en cualquier momento: `solana balance`  (objetivo: > 1.5 SOL)

---

## §1 — Desplegar el programa (Milestone 1)
1. Abrir el proyecto `lukash` en https://beta.solpg.io (si no está, recrear: *Create new project* →
   Anchor → pegar `lib.rs` → **Build** hasta "Build successful").
2. Wallet conectada (abajo-izq) y en **devnet**. Saldo > 1.5 SOL (ver §0).
3. Barra izquierda → ícono **Deploy** (cohete) → **Deploy**. Espera ~30-60s.
4. Copiar el **Program Id** que devuelve. Anotarlo abajo en §4.

> El `declare_id!` del código es un placeholder; Playground sincroniza el Program Id real al desplegar.

---

## §2 — Crear el token $LUKA (SPL) para VERLO en el explorador
> Se usa **Token-2022 con metadata on-chain** (nombre y símbolo quedan on-chain, sin hosting).
> El **logo** es opcional y va en §3 (necesita una URL pública). Sin logo igual se ve "LUKASH / LUKA".
> Todo se corre en la **terminal de Playground**.

```bash
# 1) Crear el mint con extensión de metadata (Token-2022), 6 decimales
spl-token create-token --program-2022 --enable-metadata --decimals 6
#    -> anota el MINT ADDRESS que imprime (empieza con letras/números)

# 2) Inicializar la metadata on-chain: NOMBRE, SÍMBOLO, URI (deja la URI de §3 o "" por ahora)
spl-token initialize-metadata <MINT> "LUKASH" "LUKA" "https://raw.githubusercontent.com/doblemasterinc/lukash-brand/main/luka.json"

# 3) Crear tu cuenta asociada para el token
spl-token create-account <MINT>

# 4) Acuñar el supply total: 10,000,000,000 (10B)
spl-token mint <MINT> 10000000000

# 5) (Opcional pero recomendado) Verificar
spl-token supply <MINT>
spl-token accounts
```

**Verlo en el explorador:** abrir `https://explorer.solana.com/address/<MINT>?cluster=devnet`.
Debe mostrar **LUKASH (LUKA)**, 6 decimales, supply 10B. (El logo aparece solo si §3 está hecho.)

> Si `--enable-metadata` no está soportado por la versión de `spl-token` de Playground, avísame:
> caemos al camino clásico SPL + Metaplex Token Metadata (unos comandos más).

---

## §3 — Logo del token (opcional, cuando quieras que salga el isotipo)
El explorador lee el logo desde la **URI** de la metadata → un JSON público → campo `image`.
La URI apunta a `luka.json` (ya preparado en `brand/token-metadata/luka.json`). Falta **hostearlo público**
porque `lukash-protocol` es privado. Opciones (elige una):
- **Repo público mínimo** `lukash-brand`: subir `luka.json` + `isotipo.png` → usar las URLs `raw.githubusercontent.com`.
- **Vercel** (ya lo usas para OILO): subir los 2 archivos a un proyecto estático → usar esas URLs.
- **IPFS gratis** (Pinata/web3.storage): subir imagen y JSON → usar el gateway `https://ipfs.io/ipfs/...`.

Pasos una vez hosteado:
1. Editar `brand/token-metadata/luka.json` → poner la URL pública real del isotipo en `"image"`.
2. Subir `luka.json` al host → obtener su URL pública.
3. Actualizar la URI on-chain:
```bash
spl-token update-metadata <MINT> uri "https://<tu-host>/luka.json"
```

---

## §4 — Registro
- **Token $LUKA creado en devnet: 2026-08-20** ✅
  - Mint: `2DatjaKezpYkB3TitgwYGvpwTAWiFxN4JEwpYnk3Luvr` (Token clásico SPL, 6 dec, supply 10B)
  - Mint Authority: `5PMsqNBZ5cQWRLLU9XihEe8cp2kxka9xdK9pRNsg9xBf` (wallet Playground)
  - Explorer: https://explorer.solana.com/address/2DatjaKezpYkB3TitgwYGvpwTAWiFxN4JEwpYnk3Luvr?cluster=devnet
  - Estado: "Unknown Token" — falta metadata (nombre/logo) vía Metaplex (ver §3 + nota abajo).
- **Programa (deploy): PENDIENTE** — requiere 2.12 SOL, faucet devnet en 429 el 2026-08-20. Reintentar otro día.

### Nota metadata (por qué no se hizo con spl-token)
La versión de `spl-token` de Playground NO soporta `--enable-metadata` ni `--program-2022` (es vieja). El token
se creó clásico SPL. Para nombre/símbolo/logo hay que usar **Metaplex Token Metadata** (no el CLI):
- La **mint authority** es la wallet de Playground → cualquier herramienta que ponga metadata debe firmar con esa wallet.
- Opción recomendada: **metaboss** local con el keypair exportado de Playground, O un script TS de Metaplex en el
  cliente de Playground. Requiere primero **hostear** `luka.json` + isotipo público (ver §3).
