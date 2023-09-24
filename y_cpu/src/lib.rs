use symbols::Symbol;

pub mod banking;
pub mod devices;
pub mod emulation;
pub mod symbols;

#[derive(Debug, Clone)]
pub struct Instruction {
    pub code: OpCode,
    pub options: OpOptions,
    pub arg1: Symbol,
    pub arg2: Symbol,
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

    pub fn encode(&self) -> [u8; 3] {
        [
            self.options.encode() << 4 | self.code.encode(),
            self.arg1.address(),
            self.arg2.address(),
        ]
    }

    pub fn decode(raw: [u8; 3]) -> Self {
        Self {
            code: OpCode::decode(raw[0]),
            options: OpOptions::decode(raw[0]),
            arg1: Symbol::Resolved(raw[1]),
            arg2: Symbol::Resolved(raw[2]),
        }
    }
}

impl OpCode {
    fn encode(self) -> u8 {
        self as u8
    }

    fn decode(raw: u8) -> Self {
        match 0b0000_1111 & raw {
            0b0000 => Self::NoOp,
            0b0001 => Self::And,
            0b0010 => Self::Or,
            0b0011 => Self::Not,
            0b0100 => Self::Add,
            0b0101 => Self::Sub,
            0b0110 => Self::Mul,
            0b0111 => Self::Div,
            0b1000 => Self::SL,
            0b1001 => Self::SR,
            0b1010 => Self::RL,
            0b1011 => Self::RR,
            0b1100 => Self::Copy,
            0b1101 => Self::CompEq,
            0b1110 => Self::CompGt,
            0b1111 => Self::CompLt,
            _ => panic!("Invalid opcode (This can never happen!)"),
        }
    }
}

impl OpOptions {
    pub fn arg1_signed(&self) -> bool {
        self.0 & 0b0010_0000 == 0b0010_0000
    }

    pub fn arg2_signed(&self) -> bool {
        self.0 & 0b0001_0000 == 0b0001_0000
    }

    pub fn halt_on_error(&self) -> bool {
        self.0 & 0b1000_0000 == 0b1000_0000
    }

    fn encode(self) -> u8 {
        self.0
    }

    fn decode(raw: u8) -> Self {
        Self(raw >> 4)
    }
}
