# Public Accumulator And VCO Final State

After the final Vaddc:

- v13: cause-known whole-register Unavailable with 256 Vaddc causes back to
  Vsub;
- v14: Available from aligned Lqv at DMEM local address `0x000`;
- accumulator low: cause-known Unavailable from the final Vaddc;
- accumulator high/middle: preserved Unavailable / `ConstructionOrReset`;
- VCO carry/borrow: cause-known Unavailable from final Vaddc operands;
- VCO not-equal: Available zero;
- VCC: preserved Unavailable / `ConstructionOrReset`;
- VCE: preserved Unavailable / `ConstructionOrReset`.

No later bounded instruction consumes accumulator, VCO, VCC, or VCE truth.
