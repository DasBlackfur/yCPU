use std::{collections::HashMap, env, fs};

use y_cpu::{symbols::SymbolTable, Instruction};

fn main() {
    let input_path = env::args().nth(1).expect("Missing input file path");
    let output_path = input_path.replace(".ysm", ".bin");

    let mut instructions = Vec::new();
    let mut symbols = SymbolTable::new();
    let mut pos = 0_u8;

    let ysm = fs::read_to_string(input_path).unwrap();
    for (line, line_no) in ysm.lines().zip(1..) {
        if line.starts_with("//") || line.is_empty() {
            continue;
        }

        if let Some(name) = line.strip_prefix('$') {
            symbols.insert(name, pos - 3);
        } else {
            let instr = Instruction::from_text(line)
                .unwrap_or_else(|| panic!("Unknown instruction on line {line_no}"));
            instructions.push(instr);
            pos += 3;
        }
    }

    for Instruction { arg1, arg2, .. } in &mut instructions {
        arg1.resolve(&symbols);
        arg2.resolve(&symbols);
    }

    let output = instructions
        .iter()
        .flat_map(Instruction::encode)
        .collect::<Vec<_>>();

    fs::write(output_path, output).unwrap();
}
