use std::{collections::HashMap, env, fs};

pub enum AssemblerOptions {}

#[derive(Debug, Clone)]
pub enum Instruction {
    NoOp(bool, bool, bool, bool, Symbol, Symbol),
    And(bool, bool, bool, bool, Symbol, Symbol),
    Or(bool, bool, bool, bool, Symbol, Symbol),
    Not(bool, bool, bool, bool, Symbol, Symbol),
    Add(bool, bool, bool, bool, Symbol, Symbol),
    Sub(bool, bool, bool, bool, Symbol, Symbol),
    Mul(bool, bool, bool, bool, Symbol, Symbol),
    Div(bool, bool, bool, bool, Symbol, Symbol),
    SL(bool, bool, bool, bool, Symbol, Symbol),
    SR(bool, bool, bool, bool, Symbol, Symbol),
    RL(bool, bool, bool, bool, Symbol, Symbol),
    RR(bool, bool, bool, bool, Symbol, Symbol),
    Copy(bool, bool, bool, bool, Symbol, Symbol),
    CompEq(bool, bool, bool, bool, Symbol, Symbol),
    CompGt(bool, bool, bool, bool, Symbol, Symbol),
    CompLt(bool, bool, bool, bool, Symbol, Symbol),
    Symbol(Symbol),
}

impl Instruction {
    pub fn from_text(string: &str) -> Option<Instruction> {
        let mut options = [false; 4];
        let mut symbols = Vec::new();
        for mut part in string.split(" ") {
            if part.starts_with(":") {
                part = part.trim_matches(':');
                for option in part.split("").filter(|&x| !x.is_empty()).enumerate() {
                    if option.1 == "1" {
                        options[option.0] = true;
                    } else {
                        options[option.0] = false;
                    }
                }
            } else if part.starts_with("#") {
                symbols.push(Symbol::Resolved(
                    u8::from_str_radix(part.trim_matches('#'), 16).unwrap(),
                ));
            } else if part.starts_with("$") {
                symbols.push(Symbol::UnResolved(part.to_owned()));
            }
        }
        match string.split(" ").collect::<Vec<&str>>()[0] {
            "NOOP" => Some(Instruction::NoOp(
                options[0],
                options[1],
                options[2],
                options[3],
                symbols.remove(0),
                symbols.remove(0),
            )),
            "AND" => Some(Instruction::And(
                options[0],
                options[1],
                options[2],
                options[3],
                symbols.remove(0),
                symbols.remove(0),
            )),
            "OR" => Some(Instruction::Or(
                options[0],
                options[1],
                options[2],
                options[3],
                symbols.remove(0),
                symbols.remove(0),
            )),
            "NOT" => Some(Instruction::Not(
                options[0],
                options[1],
                options[2],
                options[3],
                symbols.remove(0),
                symbols.remove(0),
            )),
            "ADD" => Some(Instruction::Add(
                options[0],
                options[1],
                options[2],
                options[3],
                symbols.remove(0),
                symbols.remove(0),
            )),
            "SUB" => Some(Instruction::Sub(
                options[0],
                options[1],
                options[2],
                options[3],
                symbols.remove(0),
                symbols.remove(0),
            )),
            "MUL" => Some(Instruction::Mul(
                options[0],
                options[1],
                options[2],
                options[3],
                symbols.remove(0),
                symbols.remove(0),
            )),
            "DIV" => Some(Instruction::Div(
                options[0],
                options[1],
                options[2],
                options[3],
                symbols.remove(0),
                symbols.remove(0),
            )),
            "SL" => Some(Instruction::SL(
                options[0],
                options[1],
                options[2],
                options[3],
                symbols.remove(0),
                symbols.remove(0),
            )),
            "SR" => Some(Instruction::SR(
                options[0],
                options[1],
                options[2],
                options[3],
                symbols.remove(0),
                symbols.remove(0),
            )),
            "RL" => Some(Instruction::RL(
                options[0],
                options[1],
                options[2],
                options[3],
                symbols.remove(0),
                symbols.remove(0),
            )),
            "RR" => Some(Instruction::RR(
                options[0],
                options[1],
                options[2],
                options[3],
                symbols.remove(0),
                symbols.remove(0),
            )),
            "COPY" => Some(Instruction::Copy(
                options[0],
                options[1],
                options[2],
                options[3],
                symbols.remove(0),
                symbols.remove(0),
            )),
            "COMPEQ" => Some(Instruction::CompEq(
                options[0],
                options[1],
                options[2],
                options[3],
                symbols.remove(0),
                symbols.remove(0),
            )),
            "COMPGT" => Some(Instruction::CompGt(
                options[0],
                options[1],
                options[2],
                options[3],
                symbols.remove(0),
                symbols.remove(0),
            )),
            "COMPLE" => Some(Instruction::CompLt(
                options[0],
                options[1],
                options[2],
                options[3],
                symbols.remove(0),
                symbols.remove(0),
            )),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Symbol {
    Resolved(u8),
    UnResolved(String),
}

impl Symbol {
    pub fn get_address(&self) -> u8 {
        match self {
            Symbol::Resolved(some) => *some,
            Symbol::UnResolved(name) => panic!("Unresolvable symbol {}", name),
        }
    }
}

fn main() {
    let input_path: Vec<String> = env::args().collect();

    let file_string = fs::read_to_string(&input_path[1]).unwrap();

    let mut instruction_list: Vec<Instruction> = Vec::new();

    for line in file_string.split("\n").enumerate() {
        if line.1.starts_with("//") || line.1 == "" {
            continue;
        } else if line.1.starts_with("$") {
            instruction_list.push(Instruction::Symbol(Symbol::UnResolved(line.1.to_owned())));
        } else {
            instruction_list.push(
                Instruction::from_text(line.1)
                    .unwrap_or_else(|| panic!("Unknown instruction on line {}", (line.0 + 1))),
            )
        }
    }
    let mut symbol_ref_list = HashMap::new();
    let mut pos = 0;
    for symbol in instruction_list.iter() {
        match symbol {
            Instruction::Symbol(sym) => match sym {
                Symbol::Resolved(_) => todo!(),
                Symbol::UnResolved(name) => {
                    symbol_ref_list.insert(name, pos - 3);
                }
            },
            _ => pos += 3,
        }
    }
    let mut instruction_list = instruction_list.clone();
    //instruction_list.iter_mut().map(|inst| inst.symbol_mut().0 = &mut Symbol::Resolved(0));
    for inst in instruction_list.iter_mut() {
        match inst {
            Instruction::NoOp(_, _, _, _, sym1, sym2) => {
                resolve_symbols(&symbol_ref_list, sym1, sym2)
            }
            Instruction::And(_, _, _, _, sym1, sym2) => {
                resolve_symbols(&symbol_ref_list, sym1, sym2)
            }
            Instruction::Or(_, _, _, _, sym1, sym2) => {
                resolve_symbols(&symbol_ref_list, sym1, sym2)
            }
            Instruction::Not(_, _, _, _, sym1, sym2) => {
                resolve_symbols(&symbol_ref_list, sym1, sym2)
            }
            Instruction::Add(_, _, _, _, sym1, sym2) => {
                resolve_symbols(&symbol_ref_list, sym1, sym2)
            }
            Instruction::Sub(_, _, _, _, sym1, sym2) => {
                resolve_symbols(&symbol_ref_list, sym1, sym2)
            }
            Instruction::Mul(_, _, _, _, sym1, sym2) => {
                resolve_symbols(&symbol_ref_list, sym1, sym2)
            }
            Instruction::Div(_, _, _, _, sym1, sym2) => {
                resolve_symbols(&symbol_ref_list, sym1, sym2)
            }
            Instruction::SL(_, _, _, _, sym1, sym2) => {
                resolve_symbols(&symbol_ref_list, sym1, sym2)
            }
            Instruction::SR(_, _, _, _, sym1, sym2) => {
                resolve_symbols(&symbol_ref_list, sym1, sym2)
            }
            Instruction::RL(_, _, _, _, sym1, sym2) => {
                resolve_symbols(&symbol_ref_list, sym1, sym2)
            }
            Instruction::RR(_, _, _, _, sym1, sym2) => {
                resolve_symbols(&symbol_ref_list, sym1, sym2)
            }
            Instruction::Copy(_, _, _, _, sym1, sym2) => {
                resolve_symbols(&symbol_ref_list, sym1, sym2)
            }
            Instruction::CompEq(_, _, _, _, sym1, sym2) => {
                resolve_symbols(&symbol_ref_list, sym1, sym2)
            }
            Instruction::CompGt(_, _, _, _, sym1, sym2) => {
                resolve_symbols(&symbol_ref_list, sym1, sym2)
            }
            Instruction::CompLt(_, _, _, _, sym1, sym2) => {
                resolve_symbols(&symbol_ref_list, sym1, sym2)
            }
            Instruction::Symbol(_) => continue,
        }
    }
    let mut output: Vec<u8> = Vec::new();
    for inst in instruction_list {
        match inst {
            Instruction::NoOp(o1, o2, o3, o4, sym1, sym2) => output.append(&mut Vec::from([
                (options_as_u8(o1, o2, o3, o4) | 0b0000_0000),
                sym1.get_address(),
                sym2.get_address(),
            ])),
            Instruction::And(o1, o2, o3, o4, sym1, sym2) => output.append(&mut Vec::from([
                (options_as_u8(o1, o2, o3, o4) | 0b0000_0001),
                sym1.get_address(),
                sym2.get_address(),
            ])),
            Instruction::Or(o1, o2, o3, o4, sym1, sym2) => output.append(&mut Vec::from([
                (options_as_u8(o1, o2, o3, o4) | 0b0000_0010),
                sym1.get_address(),
                sym2.get_address(),
            ])),
            Instruction::Not(o1, o2, o3, o4, sym1, sym2) => output.append(&mut Vec::from([
                (options_as_u8(o1, o2, o3, o4) | 0b0000_0011),
                sym1.get_address(),
                sym2.get_address(),
            ])),
            Instruction::Add(o1, o2, o3, o4, sym1, sym2) => output.append(&mut Vec::from([
                (options_as_u8(o1, o2, o3, o4) | 0b0000_0100),
                sym1.get_address(),
                sym2.get_address(),
            ])),
            Instruction::Sub(o1, o2, o3, o4, sym1, sym2) => output.append(&mut Vec::from([
                (options_as_u8(o1, o2, o3, o4) | 0b0000_0101),
                sym1.get_address(),
                sym2.get_address(),
            ])),
            Instruction::Mul(o1, o2, o3, o4, sym1, sym2) => output.append(&mut Vec::from([
                (options_as_u8(o1, o2, o3, o4) | 0b0000_0110),
                sym1.get_address(),
                sym2.get_address(),
            ])),
            Instruction::Div(o1, o2, o3, o4, sym1, sym2) => output.append(&mut Vec::from([
                (options_as_u8(o1, o2, o3, o4) | 0b0000_0111),
                sym1.get_address(),
                sym2.get_address(),
            ])),
            Instruction::SL(o1, o2, o3, o4, sym1, sym2) => output.append(&mut Vec::from([
                (options_as_u8(o1, o2, o3, o4) | 0b0000_1000),
                sym1.get_address(),
                sym2.get_address(),
            ])),
            Instruction::SR(o1, o2, o3, o4, sym1, sym2) => output.append(&mut Vec::from([
                (options_as_u8(o1, o2, o3, o4) | 0b0000_1001),
                sym1.get_address(),
                sym2.get_address(),
            ])),
            Instruction::RL(o1, o2, o3, o4, sym1, sym2) => output.append(&mut Vec::from([
                (options_as_u8(o1, o2, o3, o4) | 0b0000_1010),
                sym1.get_address(),
                sym2.get_address(),
            ])),
            Instruction::RR(o1, o2, o3, o4, sym1, sym2) => output.append(&mut Vec::from([
                (options_as_u8(o1, o2, o3, o4) | 0b0000_1011),
                sym1.get_address(),
                sym2.get_address(),
            ])),
            Instruction::Copy(o1, o2, o3, o4, sym1, sym2) => output.append(&mut Vec::from([
                (options_as_u8(o1, o2, o3, o4) | 0b0000_1100),
                sym1.get_address(),
                sym2.get_address(),
            ])),
            Instruction::CompEq(o1, o2, o3, o4, sym1, sym2) => output.append(&mut Vec::from([
                (options_as_u8(o1, o2, o3, o4) | 0b0000_1101),
                sym1.get_address(),
                sym2.get_address(),
            ])),
            Instruction::CompGt(o1, o2, o3, o4, sym1, sym2) => output.append(&mut Vec::from([
                (options_as_u8(o1, o2, o3, o4) | 0b0000_1110),
                sym1.get_address(),
                sym2.get_address(),
            ])),
            Instruction::CompLt(o1, o2, o3, o4, sym1, sym2) => output.append(&mut Vec::from([
                (options_as_u8(o1, o2, o3, o4) | 0b0000_1111),
                sym1.get_address(),
                sym2.get_address(),
            ])),
            Instruction::Symbol(_) => continue,
        }
    }
    fs::write(
        format!("./{}", input_path[1].replace(".ysm", ".bin")),
        output,
    )
    .unwrap();
}

pub fn resolve_symbols(
    symbol_ref_list: &HashMap<&String, i32>,
    sym1: &mut Symbol,
    sym2: &mut Symbol,
) {
    match sym1 {
        Symbol::Resolved(_) => return,
        Symbol::UnResolved(name) => {
            *sym1 = Symbol::Resolved(symbol_ref_list[name].try_into().unwrap())
        }
    }
    match sym2 {
        Symbol::Resolved(_) => return,
        Symbol::UnResolved(name) => {
            *sym2 = Symbol::Resolved(symbol_ref_list[name].try_into().unwrap())
        }
    }
}

pub fn options_as_u8(
    halt_on_error: bool,
    store_debug_info: bool,
    arg1_signed: bool,
    arg2_signed: bool,
) -> u8 {
    ((halt_on_error as u8) << 7)
        | ((store_debug_info as u8) << 6)
        | ((arg1_signed as u8) << 5)
        | ((arg2_signed as u8) << 4)
}
