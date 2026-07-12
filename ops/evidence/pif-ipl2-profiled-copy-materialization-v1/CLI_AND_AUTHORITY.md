# CLI And Authority

The no-window syntax is:

    fn64_boot_probe <rom-path> --pif-rom <path> --pif-profile <ntsc-pinned|pal-pinned|mpal-pinned>

The existing optional `--max-steps` remains independent. `--pif-rom` and
`--pif-profile` are a required pair. A missing side, missing value, duplicate,
or unsupported profile is an explicit usage failure. There is no default or
`auto` profile.

Host ownership is limited to parsing the explicit token and path, reading that
one path, reporting read failure, transferring owned bytes, and presenting
Machine observations. `fn64-core` owns the profile enum and mapping, accepted
bytes, validation, copy creation, provenance, lifecycle, and failure.

The CLI performs no search, fallback, download, discovery, hash lookup, content
classification, or compatibility selection. It does not print a successful
PIF path or any PIF byte. Output distinguishes accepted input, explicit
profile, materialized ranges, and the fact that neither IPL1 nor IPL2 executed.

`LIVE_REPO_FACT`: the no-PIF invocation remains valid and reports absent input,
no search/default, and unavailable SP IMEM production.
