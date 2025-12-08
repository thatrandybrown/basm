# basm
an assembly machine

## Structure

8 8bit registers
256 memory entries

## Commands

ADD: 00aaabbb: add b to a, store in a
LOAD: 01aaabbb: load memory address at registry b into register a
STORE: 10aaabbb: write value at register a to memory address at register b
BRANCH NOT EQUAL: if equal, skip next instruction, if not, write next instruction to PC
