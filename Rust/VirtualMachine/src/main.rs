use rfd::FileDialog;
use std::fs::File;
use std::io::prelude::*;

mod instruction;
mod machine;
mod assembler;

const HERZ: u16 = 500;

fn main() {
    let path = FileDialog::new().pick_file().unwrap();

    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(reason) => panic!("Couldn't open {}: {}", display, reason),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut asm = String::new();
    match file.read_to_string(&mut asm) {
        Err(reason) => panic!("Couldn't read {}: {}", display, reason),
        Ok(_) => {/*Do nothing*/},
    }

    let mut assembler = assembler::Assembler::new(asm);
    assembler.assemble();

    let binary: Vec<u8> = assembler.output;

    let mut machine = machine::machine::Machine::new();
    machine.set_ram(0, binary);
    machine.general_registers[1] = 5;


    machine.execute(Some(HERZ as u32));
}
