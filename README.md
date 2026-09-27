# basm
an assembly machine

## Structure

8 8bit registers
256 memory entries

## Instruction Set

Every instruction is one byte: `oo aaa bbb` — a 2-bit opcode, a 3-bit
register A index, and a 3-bit register B index.

| Opcode | Encoding   | Mnemonic | Effect                          |
|--------|------------|----------|----------------------------------|
| `00`   | `00aaabbb` | ADD      | `Ra = Ra + Rb` (wraps on overflow) |
| `01`   | `01aaabbb` | LOAD     | `Ra = memory[Rb]`                |
| `10`   | `10aaabbb` | STORE    | `memory[Rb] = Ra`                |
| `11`   | `11aaabbb` | BNE      | compare `Ra` and `Rb` |
