# Exact Raw-Zero RSP NOP

Raw word `0x00000000` alone decodes as semantic identity `Nop`. It consumes no
scalar, vector, accumulator, or flag truth and writes no register, memory, or
device.

Other opcode-zero/function-zero encodings reject as unsupported scalar SLL.
General SLL was not added.
