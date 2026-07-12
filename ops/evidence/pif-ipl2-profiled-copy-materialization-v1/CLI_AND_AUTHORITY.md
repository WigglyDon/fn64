# CLI And Authority

The no-window syntax is:

    fn64_boot_probe <rom-path> [--pif-rom <path>] [--pif-profile <ntsc-pinned|pal-pinned|mpal-pinned>]

The existing optional `--max-steps` remains independent. `--pif-rom` alone
opens exactly the supplied path and transfers bytes to Machine validation. An
accepted 1,984-byte input is retained without a selected profile or SP IMEM
materialization. `--pif-profile` without `--pif-rom` is an explicit pre-input
usage failure. Missing values, duplicates, and unsupported profile spellings
also fail explicitly. There is no default or `auto` profile.

Host ownership is limited to parsing the explicit token and path, reading that
one path, reporting read failure, transferring owned bytes, and presenting
Machine observations. `fn64-core` owns the profile enum and mapping, accepted
bytes, validation, copy creation, provenance, lifecycle, and failure.

Both options together retain exact profiled materialization. The CLI performs
no environment or current-directory search, fallback, download, discovery,
hash lookup, filename inference, content classification, or compatibility
selection. It does not print a successful PIF path or any PIF byte. Output
distinguishes accepted unprofiled input, explicit profile, materialized ranges,
and the fact that neither IPL1 nor IPL2 executed.

`LIVE_REPO_FACT`: the no-PIF invocation remains valid and reports absent input,
no search/default, and unavailable SP IMEM production.
