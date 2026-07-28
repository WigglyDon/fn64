# Shared CPU/RSP SP DMA Policy

CPU `SP_RD_LEN` stores and RSP `Mtc0 SP_RD_LEN` call the same private planning
and application helpers for length/count/skip decode, capacity/address/range
preflight, byte addressing, provenance, record creation, and register
evolution. RSP contributes only its typed trigger. No algorithm is duplicated
in `rsp.rs` and no generic DMA framework is added.
