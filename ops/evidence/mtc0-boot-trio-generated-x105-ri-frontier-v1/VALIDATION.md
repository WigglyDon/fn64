# Validation

Validation is recorded against the final committed candidate, a clean checkout,
and integrated canonical main. Required gates are formatting, clippy with
warnings denied, nonzero focused filters, all Rust tests, both existing probes,
`rust/verify-forward`, context verification, fleet diagnostics, empty queue,
and local Markdown links.

The final external artifact carries literal commands, tested SHAs, Context-SHA
values, outputs, cleanup evidence, and canonical push proof. No terminal-wall
logs are committed here.
