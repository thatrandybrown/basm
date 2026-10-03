# basm
an assembly machine

## Structure

- 8 general-purpose 8-bit registers (`R0`–`R7`)
- 256 bytes of memory, shared by program and data
- 8-bit program counter (PC)

## Instruction Set

Every instruction is one byte: `oo aaa bbb` — a 2-bit opcode, a 3-bit
register A index, and a 3-bit register B index.

| Opcode | Encoding   | Mnemonic | Effect                          |
|--------|------------|----------|----------------------------------|
| `00`   | `00aaabbb` | ADD      | `Ra = Ra + Rb` (wraps on overflow) |
| `01`   | `01aaabbb` | LOAD     | `Ra = memory[Rb]`                |
| `10`   | `10aaabbb` | STORE    | `memory[Rb] = Ra`                |
| `11`   | `11aaabbb` | BNE      | compare `Ra` and `Rb`, see below |

### BNE

`BNE` also consumes the byte in memory immediately after the
instruction, which is treated as a jump target address, not as a separate
instruction.

- If `Ra == Rb`: PC is set to that byte's value (jump taken).
- If `Ra != Rb`: PC advances past that byte without jumping (fall through).

## Running

```
cargo run -- <bytes>            # decimal, comma-separated
cargo run -- --hex <bytes>      # hex, comma-separated
cargo run -- --repl [--hex]     # read one program per line from stdin
```

A program is a comma-separated list of byte values, loaded into memory
starting at address 0. Execution runs until PC reaches the end of the
loaded bytes. The final register state is printed.
