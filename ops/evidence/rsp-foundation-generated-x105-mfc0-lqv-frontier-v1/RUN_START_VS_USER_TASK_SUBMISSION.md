# Run Start Versus User Task Submission

The public x105 halt-clear at CPU PC `0xA4000508` is a general
`PUBLIC_X105_RSP_BOOT_RUN_START`. It is not an OSTask or user-task submission.

The previously accepted later user-cartridge first-task state remains a
separate higher-level CPU/SP fact created after task data and microcode have
been prepared through represented DMA. A later task halt-clear may coexist
with general run-start lineage, but the two facts do not share ownership of
halt, SP PC, RSP registers, or execution authorization.
