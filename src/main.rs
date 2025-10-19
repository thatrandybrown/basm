const NUM_REGISTERS: usize = 8;
const MEMORY_SIZE: usize = 256;

struct VirtualMachine {
    /// General-purpose registers R0 through R7
    registers: [u8; NUM_REGISTERS],
    /// Program Counter - holds the address of the next instruction to execute
    pc: u8,
    /// 256 bytes of RAM
    memory: [u8; MEMORY_SIZE],
}

fn main() {
    let vm = VirtualMachine {registers: [0; NUM_REGISTERS], pc: 0, memory: [0; MEMORY_SIZE]};
    println!("Created a new Virtual Machine!");
    println!("PC start: {}", vm.pc);
    println!("Register 0: {}", vm.registers[0]);
}
