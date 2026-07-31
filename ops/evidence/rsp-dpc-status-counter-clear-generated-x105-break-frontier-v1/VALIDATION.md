# Validation

- focused nonzero filters: dpc 8; dpc_status 4; dpc_counter 2; dpc_clock 1;
  dpc_tmem 1; dpc_pipe 1; dpc_command 1; counter_clear 2; rsp 37; rsp_mtc0 5;
  mtc0 14; rsp_fetch 1; break 3; processor_arbiter 1; processor_turn 2;
  machine_step 15; provenance 21; source_knownness 2; reset 30; bootstrap 48;
  failed_bootstrap 3; cpu_count 1; vi_cadence 1; public_x105 1;
- formatting: pass;
- Clippy workspace/all-targets with warnings denied: pass;
- fn64-core: 643/643;
- fn64-inspection library: 16/16;
- user-cartridge probe tests: 2/2;
- CLI integration: 11/11;
- doc tests: 0 failures;
- no-window Machine probe: construct/reset/no-window/result all ok;
- stable no-window step probe: 209/209 cases; result ok;
- complete Rust forward gate: `forward gate: ok`;
- context/local links: 15 checks, 0 errors;
- candidate Context-SHA:
  `ace8df8692a495c5485b6144df672a5244779e634651aa49af94b9bd52f920e0`;
- fleet: 52/52;
- integration queue: ok;
- candidate public DPC_STATUS/counter-clear/Break-frontier gate: 1/1.

The exact committed candidate SHA/tree, exact-clean result, canonical result,
patch reproduction, and artifact checksum are recorded in the final packet
artifact after those later gates complete.
