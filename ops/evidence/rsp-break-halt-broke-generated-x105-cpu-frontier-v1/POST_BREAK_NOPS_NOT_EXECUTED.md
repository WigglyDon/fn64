# Post-Break NOPs not executed

The raw-zero words at local RSP PCs `0x0a0` and `0x0a4` are present in the
public sequence but neither executes:

- Break leaves the processor token on CPU;
- `Sp` halt is true;
- the public gate stops without another `Machine::step`;
- RSP count remains 1092.

Post-Break PC state is not evidence of successor execution.

