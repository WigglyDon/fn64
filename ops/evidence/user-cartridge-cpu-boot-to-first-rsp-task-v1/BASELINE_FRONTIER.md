# Accepted-base frontier

The accepted base could structurally load and normalize an explicitly supplied
cartridge, but its no-private-input cold path had no concrete public IPL2 source.
Execution therefore retained the established SP-IMEM unknown-value boundary:
the `Lw` at `0xA4000044` targeted SP IMEM offset zero and rejected before
mutation because the word was unavailable.

This pass added one explicit deterministic public x105 bootstrap source. It is
classified separately from a user-supplied raw Boot ROM, carries its own
Machine provenance, and exists only to execute the already established public
x105 path without private firmware.

The first cartridge-runtime architectural pressure recorded after handoff was
`Blez` at `0x80002EFC`. The pass continued rather than treating that ordinary
integer identity as a product boundary.
