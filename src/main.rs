const NUM_REGISTERS: usize = 8;
const MEMORY_SIZE: usize = 256;

#[derive(Debug)]
#[repr(u8)]
enum Opcode {
    ADD = 0b00, // Add Rb to Ra, store in Ra
    LOAD = 0b01, // Load value from address in Rb into Ra
    STORE = 0b10, // Store value from Ra to address in Rb
    BNE = 0b11, // Branch (skip next instruction) if Ra != Rb
}

struct VirtualMachine {
    registers: [u8; NUM_REGISTERS],
    pc: u8,
    memory: [u8; MEMORY_SIZE],
}

fn main() {
    let mut vm = VirtualMachine {registers: [0; NUM_REGISTERS], pc: 0, memory: [0; MEMORY_SIZE]};
    println!("Created a new Virtual Machine!");
    println!("PC start: {}", vm.pc);
    println!("Register 0: {}", vm.registers[0]);
    println!("Registers count: {}", vm.registers.len());

    let program = vec![];

    // write program to memory
    for (i, opcode) in program.iter().enumerate() {
        vm.memory[i] = *opcode;
    }

    // execute program using pc
    while vm.pc < program.len() as u8 {
        let instruction = vm.memory[vm.pc as usize];
        vm.pc += 1;
        // println!("Executing instruction at PC {}: {:08b}", vm.pc - 1, instruction);
        // Decode and execute instruction
        let opcode = instruction >> 6;
        let register_a = (instruction >> 3) & 0b00000111;
        let register_b = instruction & 0b00000111;
        if opcode == Opcode::ADD as u8 {
            vm.registers[register_a as usize] = vm.registers[register_a as usize] + vm.registers[register_b as usize];
        } else if opcode == Opcode::LOAD as u8 {
            vm.registers[register_a as usize] = vm.memory[vm.registers[register_b as usize] as usize];
        } else if opcode == Opcode::STORE as u8 {
            vm.memory[vm.registers[register_b as usize] as usize] = vm.registers[register_a as usize];
        } else if opcode == Opcode::BNE as u8 {
            if vm.registers[register_a as usize] == vm.registers[register_b as usize] {
                vm.pc += 1; // skip next instruction
            } else {
                vm.pc = vm.memory[vm.pc as usize]; // jump to next instruction
            }
        } else {
            println!("Unknown operation");
        }
    }

    println!("Program execution completed!");
}
