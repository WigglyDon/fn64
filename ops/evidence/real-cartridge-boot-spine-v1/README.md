# Real cartridge boot spine v1

This lane carries a private, user-supplied cartridge through Rust-owned byte
order normalization, Machine-owned IPL3 staging, and the existing public
`Machine::step` entrance. The accepted result is the smallest source-clear
BOOT-2 path: one cartridge-derived instruction commits represented machine
state from fully known operands before execution stops at the first unsupported
frontier.

Scope is limited to cartridge ownership, bootstrap staging, the source-backed
general PIF reset stack-pointer fact, explicit bootstrap GPR knownness,
instruction provenance, the existing CPU-local execution spine, and a bounded
no-window inspection shell. It does not add PIF firmware, a wholesale post-PIF
register set, SP IMEM, loads/stores, a bus, a generalized memory map, graphics,
SDL, audio, or a host runtime.

Candidate commit: `HEAD`, meaning the commit that contains this evidence. The
external archive records the immutable candidate SHA tested and packaged.

One private local runtime artifact was used directly. Its local path, digest,
size, byte order, structural metadata, and complete bounded output exist only
in `UPLOAD_ME_fn64_real_cartridge_boot_spine_v1_repair1.tar.gz`. No ROM content
entered Git or this repository evidence.

Compatibility claim: none.
