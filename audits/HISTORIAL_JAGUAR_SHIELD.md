# Historial del Jaguar Shield — arqueología v1.0 → v4.2

> Sesión 2026-08-21. Evidencia para el ADR-015.
> **Objetivo:** rastrear cuándo y cómo entraron a la documentación los conceptos
> del Seguro Anti-Exploit, Tridente Multisig y Halborn/OtterSec, para separar
> **decisiones formales del arquitecto** de **arrastres y sugerencias** que se
> colaron por inercia.

---

## 1. Seguro Anti-Exploit — el original en v1.0

Texto **literal** del `PROTOCOLO LUKASH v1.0.docx`, líneas 187-198:

> **Seguro Anti-Exploit (Respaldo de Emergencia)**
> Si el contrato sufre un ataque técnico, luego de la auditoría técnica se puede definir:
> - **Cobertura:** El Vault puede cubrir hasta el **5% de su valor total** en pérdidas.
> - **Frecuencia:** Máximo **1 vez cada 12 meses**.
> - **Condición:** Solo activo después del año 1. En el año 5, el control pasa al DAO (la comunidad).

Cuatro condiciones concretas. Todas escritas por Sebastián.

## 2. Evolución del Seguro por versión

| Versión | Estado del Seguro | Notas |
| --- | --- | --- |
| **v1.0** | ✅ Las 4 condiciones completas | Fuente original |
| **v2.1** | ❌ **Sección completa desaparece** | Se pierde en reorganización |
| v2.3 | (sin restaurar) | |
| v3.0, v3.1 | Reaparece parcialmente | Ya con "Halborn/OtterSec" añadidos |
| **v4.0, v4.1** | ✅ Restaurado: *"5% Vault, máx 1×/año, activo año 1+, control DAO año 5"* | Se recupera casi literal |
| **v4.2 (canónico actual)** | ❌ **Mutación silenciosa** | Recorta a *"Cobertura hasta 5%... Renovación anual obligatoria"*. Pierde: "1 vez cada 12 meses", "solo tras año 1", "DAO en año 5". Y cambia "1×/año" por "renovación anual" (que ya no significa lo mismo). |

**Lección:** entre v4.1 y v4.2 se perdieron restricciones importantes sin ADR
que justifique el cambio. Es un bug documental, no una decisión.

## 3. Tridente Multisig — evolución del "3-de-3"

| Versión | Texto |
| --- | --- |
| **v1.0** | *"el Tridente (Multisig) y LUKAI auditen el evento"* — sin especificar número de firmas |
| v2.1 | Sin cambio explícito |
| **v2.3** | *"Requiere aprobación **Tridente Multisig (3 de 3 firmas)**"* — **aquí entra el 3-de-3** |
| v3.0 → v4.2 | Se mantiene 3-de-3 sin cambios |

**Lección:** el "Tridente" es concepto tuyo desde v1.0. El **"3-de-3"** específico
es de v2.3 — ratificado en el ADR-015. Los 3 firmantes concretos son
decisión pendiente pre-TGE (nunca se asignaron en ninguna versión).

## 4. Halborn / OtterSec — sugerencias que se convirtieron en decisión aparente

| Versión | Mención |
| --- | --- |
| **v1.0** | 0 menciones — dice solo "auditoría técnica" |
| **v2.1** | 1 mención en una **tabla de próximos pasos** como *"Auditoría de contratos (Ottersec / Halborn)"* — sin sección dedicada, sin ADR |
| v3.0-v4.2 | Se copia por inercia a las tablas de "requisitos pre-TGE" |

**Lección:** **nunca hubo decisión formal** de contratar a Halborn u OtterSec.
Se colaron como ejemplos plausibles en v2.1 y se arrastraron. El ADR-015 los
retira y deja "auditor(a) externa a cotizar pre-TGE".

## 5. "5% de retiro por día" — no existe en ninguna versión

Búsqueda exhaustiva en todas las versiones del Protocolo (v1.0, v2.1, v2.3,
v3.0, v3.1, v3.1_CORREGIDO, v4.0, v4.1, v4.2, v4.2_FINAL, v4.3): **cero ocurrencias**.

El concepto fue una **recomendación del Informe Cuantitativo v4.2** (SIM 2)
que el análisis previo de sesión (`audits/VALIDACION_MOTORES_SIM_CONTRACT_FAITHFUL.md`
en su primera versión) arrastró como si fuera decisión del arquitecto. **Corregido
en esta sesión**: el mecanismo real del diseño es el Seguro Anti-Exploit
(producto financiero externo, no lógica de contrato) + Tridente 3-de-3 sobre
Capa 3 + Circuit Breaker con pausa de 24h.

## 6. Regla operativa a partir de aquí

Ver `learned-rules.md`: cuando algo en v4.2/v4.3 se sienta ambiguo o
incompleto, contrastar con v1.0 y v4.1 antes de asumir que la canónica es
correcta.

---
*Base documental del ADR-015 (2026-08-21).*
