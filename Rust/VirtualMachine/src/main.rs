use rfd::FileDialog;
use std::fs::File;
use std::io::prelude::*;

mod instruction;
mod machine;
mod assembler;

fn main() {
    let path = FileDialog::new().pick_file().unwrap();

    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut asm = String::new();
    match file.read_to_string(&mut asm) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, asm),
    }

    let mut assembler = assembler::Assembler::new(asm);
    assembler.assemble();

    /*let code: [instruction::Instruction; 9] = [
        //Loop
        instruction::Instruction::from_string("ldb R1 N100".to_string()).unwrap(),
        instruction::Instruction::from_string("add R1 N1".to_string()).unwrap(),
        instruction::Instruction::from_string("mov R0 R1".to_string()).unwrap(),
        instruction::Instruction::from_string("sub R0 N30".to_string()).unwrap(),
        instruction::Instruction::from_string("jmpz R0 N6".to_string()).unwrap(),
        instruction::Instruction::from_string("jmp N1".to_string()).unwrap(),
        instruction::Instruction::from_string("mov R3 N1".to_string()).unwrap(),
        instruction::Instruction::from_string("stb R1 N100".to_string()).unwrap(),
        instruction::Instruction::from_string("halt".to_string()).unwrap(),
    ];*/

    let binary: Vec<u8> = assembler.output;

    println!("{:?}", binary);

    let mut machine = machine::machine::Machine::new();
    machine.set_ram(0, binary);
    machine.set_ram(100, vec![25]);
    machine.general_registers[1] = 5;

    machine.execute(Some(10));

    println!("{}", machine.memory[100]);
}
