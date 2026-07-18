# Control-flow planning and application

Planning already separates:

- JAL target and link, with no GPR sources;
- JALR target from captured old `rs`, with PC+8 link;
- branch/JR/JALR operands from destination writes.

The defect is the later shared rejection that treats every nonzero link
destination as an input. The narrow correction removes that destination-only
gate and adds the missing JAL/JALR destination lineage description used on
successful application.

