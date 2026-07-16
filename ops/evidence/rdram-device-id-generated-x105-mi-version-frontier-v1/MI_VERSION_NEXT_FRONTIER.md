# MI_VERSION next frontier

Decision: `MI_VERSION_READ_NEXT_FRONTIER_ONLY`.

PC/next `A400016C/A4000170`; word `8C300004`, `Lw r16,4(r1)`; r1=`FFFFFFFFA4300000` from `Lui` at `A4000168`; effective/CPU/physical=`FFFFFFFFA4300004`/`A4300004`/`04300004`; result `MachineLoadWordRejectionReason::DirectTargetMiss`; Count `32160`, commits `32176`, unchanged by rejection.

The public header identifies four revision bytes, but no value or silicon revision is represented. The dependent branch is not executed.
