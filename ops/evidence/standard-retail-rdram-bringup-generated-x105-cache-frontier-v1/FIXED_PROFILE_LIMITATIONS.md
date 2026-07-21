# Fixed-Profile Limitations

This pass does not represent analog current-control physics, cycle-accurate
RDRAM timing, electrical units, asynchronous completion, a device clock,
arbitrary capacities/module sizes/manufacturers, hot-plugging, cartridge-based
configuration, or a host-selectable profile.

The current 4 MiB Machine uses one NEC/non-enhanced identity and one monotonic
digital response curve. The 8 MiB structural profile exists only as the
capacity-derived four-module alternative; the current Machine was not resized.
No general RDRAM register array, bus, MMIO framework, or physical device model
was added.
