# Validation

- focused nonzero filters: rsp 41; rsp_break 4; break 7; sp_halt 1;
  sp_broke 1; interrupt_on_break 1; mi_sp_pending 1; rsp_fetch 1;
  rsp_run_start 1; processor_arbiter 1; processor_turn 2; machine_step 15;
  provenance 22; source_knownness 2; dpc 7; dpc_status 3; reset 31;
  bootstrap 49; failed_bootstrap 4; cpu_count 1; vi_cadence 1;
  public_x105 1; post_break 1; task_completion 1;
- formatting: pass;
- Clippy workspace/all-targets with warnings denied: pass;
- fn64-core: 647/647;
- fn64-inspection library: 16/16;
- user-cartridge probe parser/redaction tests: 2/2;
- CLI integration: 11/11;
- doc tests: 0 failures;
- no-window Machine probe: construct/reset/no-window/result all ok;
- stable no-window step probe: 209/209 cases; result ok;
- complete Rust forward gate: `forward gate: ok`;
- context/local links: 15 checks, 0 errors;
- candidate Context-SHA:
  `3f4e05935f7b1004faace3d0345e5ac3a205b5f6bd409c9b0ae796710a0c19c8`;
- fleet: 52/52;
- integration queue: ok;
- candidate public Break/halt/broke/CPU-frontier gate: 1/1;
- public result: RSP count 1092, PC/next `0x0a0/0x0a4`, halt/broke true,
  interrupt-on-break false, MI SP-pending false, next CPU word
  `0x02cfb024` (`SpecialAnd`) identified and not executed.

The exact committed candidate SHA/tree, exact-clean result, patch reproduction,
canonical result, and artifact checksum are recorded in the final packet
artifact after those later gates complete.
