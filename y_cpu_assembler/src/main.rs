use std::{collections::HashMap, env, fs};

#[derive(Debug, Clone)]
pub struct Instruction {
    code: OpCode,
    options: OpOptions,
    arg1: Symbol,
    arg2: Symbol,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Symbol {
    Resolved(u8),
    UnResolved(String, i8),
}

#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    NoOp = 0b0000,
    And = 0b0001,
    Or = 0b0010,
    Not = 0b0011,
    Add = 0b0100,
    Sub = 0b0101,
    Mul = 0b0110,
    Div = 0b0111,
    SL = 0b1000,
    SR = 0b1001,
    RL = 0b1010,
    RR = 0b1011,
    Copy = 0b1100,
    CompEq = 0b1101,
    CompGt = 0b1110,
    CompLt = 0b1111,
}

#[derive(Debug, Clone, Copy)]
pub struct OpOptions(u8);

impl Instruction {
    pub fn from_text(string: &str) -> Option<Self> {
        let mut parts = string.split_whitespace();

        use OpCode::*;
        let code = match parts.next()? {
            "NOOP" => NoOp,
            "AND" => And,
            "OR" => Or,
            "NOT" => Not,
            "ADD" => Add,
            "SUB" => Sub,
            "MUL" => Mul,
            "DIV" => Div,
            "SL" => SL,
            "SR" => SR,
            "RL" => RL,
            "RR" => RR,
            "COPY" => Copy,
            "COMPEQ" => CompEq,
            "COMPGT" => CompGt,
            "COMPLE" => CompLt,
            _ => return None,
        };

        let mut args = Vec::new();
        let mut raw_options = 0b0000;

        for part in parts {
            let (prefix, value) = part.split_at(1);
            match prefix {
                ":" => {
                    for (i, option) in value.chars().enumerate() {
                        let flag = if option == '1' { 1 } else { 0 };
                        raw_options |= flag << (3 - i);
                    }
                }
                // Ideally we would not panic here but return `None` / an appropriate error
                "#" => args.push(Symbol::Resolved(u8::from_str_radix(value, 16).unwrap())),
                "$" => args.push(Symbol::UnResolved(value.to_owned(), 0)),
                "+" | "-" => match args.last_mut() {
                    Some(arg) => match arg {
                        Symbol::Resolved(_) => return None,
                        Symbol::UnResolved(_, offset) => {
                            *offset +=
                                i8::from_str_radix(&(prefix.to_owned() + value), 16).unwrap();
                        }
                    },
                    None => return None,
                },
                _ => return None,
            }
        }

        let mut args = args.into_iter();
        let arg1 = args.next()?;
        let arg2 = args.next()?;
        if args.next().is_some() {
            // more than 2 args were provided, we could return a descriptive error here
            return None;
        }

        Some(Self {
            code,
            options: OpOptions(raw_options),
            arg1,
            arg2,
        })
    }

    fn encode(&self) -> [u8; 3] {
        [
            self.options.encode() << 4 | self.code.encode(),
            self.arg1.address(),
            self.arg2.address(),
        ]
    }
}

impl OpCode {
    fn encode(self) -> u8 {
        self as u8
    }
}

impl OpOptions {
    fn encode(self) -> u8 {
        self.0
    }

    // can add helper methods if necessary in the future:
    // fn halt_on_error(self) -> bool { … }
    // fn store_debug_info(self) -> bool { … }
    // fn arg1_signed(self) -> bool { … }
    // fn arg2_signed(self) -> bool { … }
}

impl Symbol {
    pub fn address(&self) -> u8 {
        match self {
            Symbol::Resolved(addr) => *addr,
            Symbol::UnResolved(name, _) => panic!("Unresolvable symbol {name}"),
        }
    }

    fn resolve(&mut self, lookup: &SymbolTable<'_>) -> bool {
        match self {
            Symbol::Resolved(_) => true,
            Symbol::UnResolved(name, offset) => {
                if let Some(addr) = lookup.get(name.as_str()) {
                    *self = Symbol::Resolved(((*addr as i16) + (*offset as i16)) as u8);
                    true
                } else {
                    false
                }
            }
        }
    }
}

type SymbolTable<'input> = HashMap<&'input str, u8>;

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
            symbols.insert(name, pos - 3); // Are we sure it's -3 ?
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
