pub mod banking;
pub mod devices;
pub mod emulation;

#[derive(Debug)]
pub enum Instruction {
    NoOp(bool, bool, bool, bool),
    And(bool, bool, bool, bool, u8, u8),
    Or(bool, bool, bool, bool, u8, u8),
    Not(bool, bool, bool, bool, u8),
    Add(bool, bool, bool, bool, u8, u8),
    Sub(bool, bool, bool, bool, u8, u8),
    Mul(bool, bool, bool, bool, u8, u8),
    Div(bool, bool, bool, bool, u8, u8),
    SL(bool, bool, bool, bool, u8),
    SR(bool, bool, bool, bool, u8),
    RL(bool, bool, bool, bool, u8),
    RR(bool, bool, bool, bool, u8),
    Copy(bool, bool, bool, bool, u8, u8),
    CompEq(bool, bool, bool, bool, u8, u8),
    CompGt(bool, bool, bool, bool, u8, u8),
    CompLt(bool, bool, bool, bool, u8, u8),
}

impl Instruction {
    pub fn from_3bytes(bytes: [u8; 3]) -> Instruction {
        let opcode = bytes[0] & 0b0000_1111;
        let halt_on_error = bytes[0] & 0b1000_0000 == 0b1000_0000;
        let store_debug_info = bytes[0] & 0b0100_0000 == 0b0100_0000;
        let arg1_signed = bytes[0] & 0b0010_0000 == 0b0010_0000;
        let arg2_signed = bytes[0] & 0b0001_0000 == 0b0001_0000;
        let arg1 = bytes[1];
        let arg2 = bytes[2];

        match opcode {
            0 => Instruction::NoOp(halt_on_error, store_debug_info, arg1_signed, arg2_signed),
            1 => Instruction::And(
                halt_on_error,
                store_debug_info,
                arg1_signed,
                arg2_signed,
                arg1,
                arg2,
            ),
            2 => Instruction::Or(
                halt_on_error,
                store_debug_info,
                arg1_signed,
                arg2_signed,
                arg1,
                arg2,
            ),
            3 => Instruction::Not(
                halt_on_error,
                store_debug_info,
                arg1_signed,
                arg2_signed,
                arg1,
            ),
            4 => Instruction::Add(
                halt_on_error,
                store_debug_info,
                arg1_signed,
                arg2_signed,
                arg1,
                arg2,
            ),
            5 => Instruction::Sub(
                halt_on_error,
                store_debug_info,
                arg1_signed,
                arg2_signed,
                arg1,
                arg2,
            ),
            6 => Instruction::Mul(
                halt_on_error,
                store_debug_info,
                arg1_signed,
                arg2_signed,
                arg1,
                arg2,
            ),
            7 => Instruction::Div(
                halt_on_error,
                store_debug_info,
                arg1_signed,
                arg2_signed,
                arg1,
                arg2,
            ),
            8 => Instruction::SL(
                halt_on_error,
                store_debug_info,
                arg1_signed,
                arg2_signed,
                arg1,
            ),
            9 => Instruction::SR(
                halt_on_error,
                store_debug_info,
                arg1_signed,
                arg2_signed,
                arg1,
            ),
            10 => Instruction::RL(
                halt_on_error,
                store_debug_info,
                arg1_signed,
                arg2_signed,
                arg1,
            ),
            11 => Instruction::RR(
                halt_on_error,
                store_debug_info,
                arg1_signed,
                arg2_signed,
                arg1,
            ),
            12 => Instruction::Copy(
                halt_on_error,
                store_debug_info,
                arg1_signed,
                arg2_signed,
                arg1,
                arg2,
            ),
            13 => Instruction::CompEq(
                halt_on_error,
                store_debug_info,
                arg1_signed,
                arg2_signed,
                arg1,
                arg2,
            ),
            14 => Instruction::CompGt(
                halt_on_error,
                store_debug_info,
                arg1_signed,
                arg2_signed,
                arg1,
                arg2,
            ),
            15 => Instruction::CompLt(
                halt_on_error,
                store_debug_info,
                arg1_signed,
                arg2_signed,
                arg1,
                arg2,
            ),
            _ => panic!("Invalid opcode (This should never ever happen)"),
        }
    }
}

pub enum Halted {
    Running,
    Errored,
    Halted,
}
