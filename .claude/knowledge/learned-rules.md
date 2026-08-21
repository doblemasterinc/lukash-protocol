# Learned Rules — LUKASH Protocol

> Reglas aprendidas (máx 1-2 líneas c/u). Se promueven desde `tasks/lessons.md` cuando un patrón se repite 2+ veces.

- El Protocolo v4.2 es la fuente de verdad; ante conflicto Protocolo↔BMC↔presentación, gana el Protocolo.
- Al reescribir cualquier texto o UI, "Jaguar Score" → "Aura" (ADR-002). La mecánica no cambia.
- Verificar SIEMPRE que los porcentajes de composición del Vault y distribución de fees sumen 100% antes de codificar (hay una discrepancia histórica de 105% en el Vault).
- Distinguir "Fase" (1/2/3 de inversión/capital) de "Etapa" (0-4 del protocolo, condiciones on-chain). No son lo mismo.
- Distinguir estado del Motor B (B0/B2, según K vs $25M) de la Etapa del protocolo. B0 y B2 son estados, no etapas.
- Para leer .docx: `pandoc "archivo.docx" -t plain -o salida.txt`. Los .txt ya extraídos están en `extracted/`.
- No lanzar subagentes a leer los .docx/.txt gigantes en loop: el watchdog los cuelga. Extraer a texto acotado primero.
- De cara al usuario el Vault se llama **"Reserva Sagrada"** (colectiva del ecosistema); NUNCA "tu Reserva" personal, ni "Reserva común", ni "KASH"/"Vault". El eje personal de ADR-010 se dice como "**lo tuyo / tu $LUKA sube de valor**". Clave de marca: **cada transacción hace crecer la Reserva Sagrada → une comunidad + volumen (negocio)**. La garantiza el protocolo (Motores) de forma automática y descentralizada = "la bondad del protocolo".
- **LUKAI** = la IA que **guía y orienta** al usuario (maestro/shaman, sin jerga). Debe tener presencia en material de cara al usuario, no solo mencionarse.
- Voz/copy: dinero coloquial = **"Lukas"** (10 lukas, 20 lukas), NUNCA "plata". Token = **$LUKA**. Tono juvenil, directo, simple (no rebuscado).
