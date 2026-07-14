# Cold and NMI boundary

The bounded source relation uses zero to enter the cold path and nonzero to
select an NMI path. Official reset documentation explains that NMI resets the
CPU but not the RCP, so RI state can be retained across an NMI reset.

This pass represents only the explicitly selected cold-x105 entry and its zero
RI_SELECT fact. It does not represent NMI entry, pre-NMI, retention mutation,
nonzero RI_SELECT, reset-button behavior, or the NMI branch body.
