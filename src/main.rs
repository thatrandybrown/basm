mod basm;
use basm::VirtualMachine;

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
     *  cargo run -- --hex 40,78,37,37,37,37,37,37,0F,CE,08,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,C0,01
    **/

    let program = if hex {
        instructions
            .split(',')
            .filter_map(|s| u8::from_str_radix(s.trim(), 16).ok())
            .collect::<Vec<u8>>()
    } else {
        instructions.split(',').filter_map(|s| s.trim().parse::<u8>().ok()).collect::<Vec<u8>>()
    };

    let output: &[u8] = vm.execute(&program);

    println!("Registers: {:?}", output);
}
