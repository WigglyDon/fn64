# Validation

Validation is intentionally layered:

1. focused Mtc0, Xori, SP-DMA, atomicity, and public-x105 tests;
2. formatting and clippy with warnings denied;
3. complete fn64-core, inspection, and CLI test suites;
4. no-window Machine and step probes;
5. complete Rust forward gate;
6. context, link, fleet, and integration-queue verification;
7. the public x105 Mtc0/Xori/DMA/Lui gate on candidate, independent exact-SHA
   checkout, and integrated canonical main.

Final exact counts, tested SHAs, Context-SHA, and literal success markers are
sealed in the pass artifact after all gates complete.
