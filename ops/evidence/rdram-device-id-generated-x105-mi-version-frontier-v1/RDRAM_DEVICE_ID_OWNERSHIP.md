# RDRAM DEVICE_ID ownership

`LIVE_REPO_FACT`: the existing `Rdram` instance remains the sole Machine-owned RDRAM owner. It owns the byte vector, broadcast-delay fact, broadcast refresh-row fact, and one optional `MachineRdramBroadcastDeviceIdRequestState`.

The request contains only raw word `0x80000000`, requested base `0x02000000`, global aperture classification, and exact CPU-store provenance. There is no parallel byte store, register array, bus, module collection, relocation engine, or read route.
