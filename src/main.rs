const NUM_REGISTERS: usize = 8;
const MEMORY_SIZE: usize = 256;

#[derive(Debug)]
#[repr(u8)]
enum Opcode {
    ADD = 0b00,   // Add Rb to Ra, store in Ra
    LOAD = 0b01,  // Load value from address in Rb into Ra
    STORE = 0b10, // Store value from Ra to address in Rb
    BNE = 0b11,   // Branch if Ra != Rb
}

struct VirtualMachine {
    registers: [u8; NUM_REGISTERS],
    pc: u8,
    memory: [u8; MEMORY_SIZE],
}

fn main() {
    let mut vm = VirtualMachine {
        registers: [0; NUM_REGISTERS],
        pc: 0,
        memory: [0; MEMORY_SIZE],
    };
    let args: Vec<String> = std::env::args().collect();

    let instructions = args
        .iter()
        .skip(1)
        .find(|arg| !arg.starts_with("--"))
        .map(|s| s.as_str())
        .unwrap_or("");

    println!("Executing instructions: {}", instructions);

    println!("{:?}", instructions.as_bytes().to_vec());

    println!("Created a new Virtual Machine!");

    let program = instructions.as_bytes().to_vec();

    // let program = vec![
    //      // load 64, load 1
    //     0b01000000, 0b01111000,

    //     // set target to 6
    //     0b00110111, 0b00110111, 0b00110111, 0b00110111, 0b00110111, 0b00110111,

    //     // increment register by 1, bne, back to step 1
    //     0b00001111, 0b11001110, 0b00001000,

    //     0b11000000, 0b11000000, 0b11000000, 0b11000000, 0b11000000, 0b11000000, 0b11000000, 0b11000000,
    //     0b11000000, 0b11000000, 0b11000000, 0b11000000, 0b11000000, 0b11000000, 0b11000000, 0b11000000,
    //     0b11000000, 0b11000000, 0b11000000, 0b11000000, 0b11000000, 0b11000000, 0b11000000, 0b11000000,
    //     0b11000000, 0b11000000, 0b11000000, 0b11000000, 0b11000000, 0b11000000, 0b11000000, 0b11000000,
    //     0b11000000, 0b11000000, 0b11000000, 0b11000000, 0b11000000, 0b11000000, 0b11000000, 0b11000000,
    //     0b11000000, 0b11000000, 0b11000000, 0b11000000, 0b11000000, 0b11000000, 0b11000000, 0b11000000,
    //     0b11000000, 0b11000000, 0b11000000, 0b11000000, 0b11000000, // 0b11000000, 0b11000000, 0b11000000,
    //     // 0b11000000, 0b11000000, 0b11000000, 0b11000000, 0b11000000, // 0b11000000, // 0b11000000, // 0b11000000,

    //     0b00000001 //64
    // ];

    for (i, opcode) in program.iter().enumerate() {
        vm.memory[i] = *opcode;
    }

    while vm.pc < program.len() as u8 {
        let instruction = vm.memory[vm.pc as usize];
        vm.pc += 1;
        let opcode = instruction >> 6;
        let register_a = (instruction >> 3) & 0b00000111;
        let register_b = instruction & 0b00000111;
        if opcode == Opcode::ADD as u8 {
            vm.registers[register_a as usize] =
                vm.registers[register_a as usize] + vm.registers[register_b as usize];
        } else if opcode == Opcode::LOAD as u8 {
            vm.registers[register_a as usize] =
                vm.memory[vm.registers[register_b as usize] as usize];
        } else if opcode == Opcode::STORE as u8 {
            vm.memory[vm.registers[register_b as usize] as usize] =
                vm.registers[register_a as usize];
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

    println!("Registers: {:?}", vm.registers);
    println!("Memory: {:?}", vm.memory);
    println!("Program execution completed!");
}
