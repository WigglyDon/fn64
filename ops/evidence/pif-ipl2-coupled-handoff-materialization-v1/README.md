# Cold x105 coupled handoff reconstruction

Classification: `NTSC_PINNED_COLD_X105_MATERIALIZATION_SUPPORTED`.

This evidence reconstructs the bounded inherited state consumed by the cold
x105 IPL3 prelude. It supports one product path only: explicit raw PIF input,
`NTSC_PINNED`, explicit `X105`, explicit cold reset, explicit cartridge boot,
and an explicit PIF version bit. PAL and MPAL remain fail-closed because their
retained link values lack independent matching corroboration.

The reconstruction is source-backed materialization, not PIF, IPL1, or IPL2
execution. All numeric test inputs are generated. No private ROM or PIF input,
firmware word, copied assembly, or copied disassembly was used.

Evidence labels:

- current fn64 source and Git state: `LIVE_REPO_FACT`;
- command output at the named revision: `RUNTIME_FACT`;
- retired donor content: `WORKER_CLAIM`;
- pinned manuals, reconstructions, and emulators: `EXTERNAL_TECHNICAL_EVIDENCE`;
- arithmetic conclusions name their supporting facts as `INFERENCE`;
- unproved physical revisions remain `UNKNOWN`.
