# Machine lineage

Committed branch/jump lineage:

`generated word -> Machine fetch -> fixed-field decode -> assigned identity ->
pure identity-specific plan -> optional existing-owner link write -> CPU
ordinary-control-flow commit -> existing Count owner -> represented committed
identity`

Slot lineage:

`explicit owner context -> later public Machine::step -> ordinary represented
instruction path -> existing committed cadence -> context clear`

Exception lineage:

`explicit owner context + exact fault -> existing source-specific action ->
snapshot restoration -> shared COP0 EPC/BD selection -> existing exception-code
and BadVAddr owner -> vector + context clear`

Planning captures instruction PC, slot PC, operands, condition, target,
selected next PC, and optional link before application. The current-PC producer
test proves `JALR rs=rd` production changes no register, control-flow, Count, or
COP0 fact.
