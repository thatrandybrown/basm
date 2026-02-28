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

impl VirtualMachine {
    fn new() -> Self {
        VirtualMachine {
            registers: [0; NUM_REGISTERS],
            pc: 0,
            memory: [0; MEMORY_SIZE],
        }
    }

    fn execute(&mut self, program: &[u8]) -> &[u8; NUM_REGISTERS] {
        for (i, opcode) in program.iter().enumerate() {
            self.memory[i] = *opcode;
        }

        while self.pc < program.len() as u8 {
            let instruction = self.memory[self.pc as usize];
            self.pc += 1;
            let opcode = instruction >> 6;
            let register_a = (instruction >> 3) & 0b00000111;
            let register_b = instruction & 0b00000111;
            if opcode == Opcode::ADD as u8 {
                self.registers[register_a as usize] =
                    self.registers[register_a as usize] + self.registers[register_b as usize];
            } else if opcode == Opcode::LOAD as u8 {
                self.registers[register_a as usize] =
                    self.memory[self.registers[register_b as usize] as usize];
            } else if opcode == Opcode::STORE as u8 {
                self.memory[self.registers[register_b as usize] as usize] =
                    self.registers[register_a as usize];
            } else if opcode == Opcode::BNE as u8 {
                if self.registers[register_a as usize] == self.registers[register_b as usize] {
                    self.pc += 1; // skip next instruction
                } else {
                    self.pc = self.memory[self.pc as usize]; // jump to next instruction
                }
            } else {
                println!("Unknown operation");
            }
        }

        &self.registers
    }
}

fn main() {
    let mut vm = VirtualMachine::new();
    let args: Vec<String> = std::env::args().collect();

    let hex = args.contains(&"--hex".to_string());

    let instructions = args
        .iter()
        .skip(1)
        .find(|arg| !arg.starts_with("--"))
        .map(|s| s.as_str())
        .unwrap_or("");

    /**
     *  cargo run -- 64,120,55,55,55,55,55,55,15,206,8,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,192,1
    **/

    let program = instructions.split(',').filter_map(|s| s.trim().parse::<u8>().ok()).collect::<Vec<u8>>();

    let output: &[u8; NUM_REGISTERS] = vm.execute(&program);

    println!("Registers: {:?}", output);
}
