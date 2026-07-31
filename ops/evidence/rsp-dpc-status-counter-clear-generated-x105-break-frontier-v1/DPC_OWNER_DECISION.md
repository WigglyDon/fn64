# DPC owner decision

`Machine` owns one private `Dpc` value in `fn64-core/src/dpc.rs`. `Dpc` is a
sibling of `Sp`, `Mi`, and `Rdram`; it is not nested in RSP or SP. It owns only
four counter knowledge states:

- clock;
- command busy;
- pipe busy;
- TMEM load.

No RDP owner, device registry, generic bus, MMIO bank, or status-mode owner was
introduced.
