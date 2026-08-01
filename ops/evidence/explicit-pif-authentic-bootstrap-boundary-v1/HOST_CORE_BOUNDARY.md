# Host/core boundary

The inspection host owns only the literal path, one file read, read failure,
and owned-byte transfer. Diagnostics render the PIF input as
`<REDACTED_USER_PIF_FIRMWARE>` and never retain a path or basename.

The core receives `Vec<u8>` through `Machine::install_pif_firmware`. Existing
structural validation and typed firmware/profile state own all subsequent
classification and composition. Machine stores no filesystem path and performs
no search, download, default lookup, digest selection, or acquisition.

The user-cartridge route maps the existing
`PifIpl2ProfileRequiresFirmware` bootstrap error to a named material-owner stop
before stepping. It does not introduce a new error framework.
