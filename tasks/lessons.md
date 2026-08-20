# Lessons — LUKASH

> Nunca borrar. Cada lección registra qué pasó y qué haremos distinto. Patrón 2+ veces → promover a learned-rules.md.

## 2026-08-19 — El proyecto usa dos nombres para el mismo sistema de reputación
"Jaguar Score" (docs v4.2) vs "Aura" (término nuevo de Sebastián). Decisión: Aura es el rebrand oficial (ADR-002).
Lección: al haber muchas versiones de documentos, confirmar terminología con Sebastián antes de codificar.

## 2026-08-19 — Subagentes leyendo .docx/.txt gigantes se cuelgan
Las dos auditorías en background fallaron por watchdog (600s sin progreso) al leer archivos grandes en loop.
Lección: extraer a texto acotado primero; para docs ya leídos en contexto, auditar inline sin subagente.

## 2026-08-20 — El panel del navegador no compositaba → no se pudo compilar en Playground desde aquí
Screenshots/clics fallaban ("Browser pane is not displayed"). Solución: preparar single-file + guiar al usuario (compiló él con éxito). Lección: para compilar Anchor sin toolchain local, Solana Playground guiado por el usuario es la vía; hacer revisión "de escritorio" rigurosa antes (cacé el error de comentarios //! así).

## 2026-08-20 — Financiación con capital cero: NO diluir primero
Aprendizaje cross-proyecto: en Solana hay dinero no dilutivo (Superteam Earn/Instagrants, Solana Foundation + Finternet grants) y Colosseum da $250K pre-seed vía hackathon. El SAFE con ángeles es último recurso, no primero. Aplicable a cualquier proyecto cripto propio con poco capital.

## 2026-08-20 — Compensación de partners: tokens vesteados, no Vault Sociedad
Regla: tokens vesteados = pago por servicio (KOLs, MM). Vault Sociedad (equity) = solo para capital/largo plazo. No mezclar. Protege el patrimonio del fundador. (ADR-011 / decisión de diseño.)
