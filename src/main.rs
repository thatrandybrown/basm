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
        println!("Executing instruction at PC {}: {:08b}", vm.pc - 1, instruction);
        // Decode and execute instruction
    }

    println!("Program execution completed!");
}
