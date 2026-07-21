# Module Address Mapping

The accepted pre-start zero DEVICE_ID request temporarily maps module zero to
physical base zero. Post-start first-responder writes map module one to
`0x00200000`, probe one absent module at `0x00400000`, then stop discovery.

The global request moves represented module mappings to `0x02000000` without
copying backing bytes. Loop2 final writes map module zero to `0x00000000` and
module one to `0x00200000`. Final mapping is linear through `0x003FFFFF` and
matches the single 4 MiB backing owner. The absent probe changes no module.
