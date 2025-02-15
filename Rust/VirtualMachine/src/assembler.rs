use instruction::Instruction;
use crate::instruction;

pub struct Assembler {
    pub code: String,
    pub output: Vec<u8>,
}

impl Assembler {
    pub fn new(code: String) -> Assembler {
        Assembler{code, output: Vec::new()}
    }
    pub fn assemble(&mut self) {
        // Remove all comments and empty lines
        let code_seperated_by_lines = self.code.lines();

        let mut lines_without_comments: Vec<String> = vec![];

        for line in code_seperated_by_lines {
            if line.chars().nth(0).unwrap() != '#' && line != "" {
                let part_without_comment = line.split("#").nth(0).unwrap();
                lines_without_comments.push(part_without_comment.to_string());
            }
        }

        // Go throw every line and set all the values defined in the file

        let mut current_line: u32 = 0;
        let mut lines_except_values: Vec<String> = vec![];
        let mut replacements: Vec<(String, String)> = vec![];

        for line in lines_without_comments {
            let characters: Vec<char> = line.chars().collect();
            if *characters.last().unwrap() == ':' {
                // This indicates a function afterwords, therefore, store the current line number
                let name = characters[0..characters.len()-1].iter().collect::<String>();
                replacements.push((name.clone(), format!("N{}", current_line)));
                println!("pushing replacement {:?}", (name.clone(), format!("N{}", current_line)));
                continue;
            }else if *characters.first().unwrap() == '.' {
                let selected_chars = &characters[1..characters.len()];
                let selected_string = selected_chars.iter().collect::<String>();
                let mut selected_string_split = selected_string.split(' ');
                println!("{:?}", selected_string_split);

                let name = selected_string_split.next().unwrap();
                let replacement = selected_string_split.next().unwrap();
                replacements.push((name.to_string(), format!("{}", replacement.to_string())));
                println!("pushing replacement {:?}", (name.to_string(), format!("{}", replacement.to_string())));
                continue;
            }
            current_line += 1;
            lines_except_values.push(line.to_string());
        }

        println!("replacements: {:?}", replacements);

        // Now replace the replacements
        let replacements_length = replacements.len();
        for _ in 0..replacements_length /* Repeat the process below multiple times so everything will be replaced correctly */{
            for i in 0..replacements_length /* Loop throw every argument */ {
                for j in 0..replacements_length /* Check every argument if it needs to be replaced */ {
                    if replacements[i].0 == replacements[j].1.clone() {
                        replacements[j].1 = replacements[i].1.clone();
                    }
                }
            }
        }

        // Now replace the keywords in code

        for i in 0..lines_except_values.len() {
            for replacement in replacements.clone() {
                let replace_keyword = replacement.0.clone();
                let replace_value = replacement.1.clone();
                lines_except_values[i] = lines_except_values[i].replace::<&str>(replace_keyword.as_ref(), replace_value.as_ref());
            }
        }


        let mut binary: Vec<u8> = vec![];
        for line in lines_except_values.clone() {
            println!("{}", line);
            let mut instruction = Instruction::from_string(line).unwrap().to_binary();
            binary.append(&mut instruction);
        }

        self.output = binary;

        println!("replacements: {:?}", replacements);
        println!("code: {:?}", lines_except_values.join("\n"));

    }
}