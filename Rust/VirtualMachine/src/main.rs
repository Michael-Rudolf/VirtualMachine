mod instruction;
mod machine;


fn main() {
    let code: [instruction::Instruction; 8] = [
        //Loop
        instruction::Instruction::from_string("ldb R1 N100".to_string()).unwrap(),
        instruction::Instruction::from_string("add R1 N1".to_string()).unwrap(),
        instruction::Instruction::from_string("mov R0 R1".to_string()).unwrap(),
        instruction::Instruction::from_string("sub R0 N30".to_string()).unwrap(),
        instruction::Instruction::from_string("jmpz R0 N6".to_string()).unwrap(),
        instruction::Instruction::from_string("jmp N1".to_string()).unwrap(),
        instruction::Instruction::from_string("mov R3 N1".to_string()).unwrap(),
        instruction::Instruction::from_string("halt".to_string()).unwrap(),
    ];

    let mut binary: Vec<u8> = vec![];


    code.map(|instruction| binary.append(&mut instruction.to_binary()));
    println!("{:?}", binary);

    let mut machine = machine::machine::Machine::new();
    machine.set_ram(0, binary);
    machine.set_ram(100, vec![25]);
    machine.general_registers[1] = 5;

    // Actually important stuff asdgvasödxlf
    machine.execute(Some(5));
}


