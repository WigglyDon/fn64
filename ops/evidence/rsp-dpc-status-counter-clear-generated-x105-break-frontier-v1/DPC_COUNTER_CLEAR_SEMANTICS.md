# DPC counter clear semantics

One successful command replaces the clock and TMEM-load counter states with
Available zero. It preserves the command-busy and pipe-busy states exactly.

A repeated exact clear remains zero and replaces each selected counter's
provenance with the new committing instruction cause. There is no counter
increment or elapsed-time effect.
