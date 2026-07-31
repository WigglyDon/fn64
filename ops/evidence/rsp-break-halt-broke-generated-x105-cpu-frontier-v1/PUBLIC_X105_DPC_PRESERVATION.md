# Public x105 DPC preservation

Break does not read or mutate DPC counter truth:

- clock: Available zero with the accepted DPC_STATUS clear provenance;
- TMEM-load: Available zero with the accepted DPC_STATUS clear provenance;
- command-busy: preserved Unavailable;
- pipe-busy: preserved Unavailable.

No DPC mode, readback, counter cadence, RDP owner, or RDP execution is created.

