use crate::{banking::Banker, devices::Device, symbols::Symbol, Instruction, OpOptions};

pub struct CPU {
    pub reg_zero: u8,
    pub inst_mem: Banker<[u8; 127]>,
    pub data_mem: Banker<[u8; 64]>,
    pub devices: Vec<Box<dyn Device>>,
}

pub enum Halted {
    Running,
    Halted,
}

impl CPU {
    pub fn new(inst_mem: [u8; 127], devices: Vec<Box<dyn Device>>) -> CPU {
        let mut mapped_devices = Vec::with_capacity(62);
        for device in devices.into_iter() {
            let address = device.address() as usize - 194;
            mapped_devices[address] = device;
        }
        CPU {
            reg_zero: 0,
            inst_mem: Banker::new(inst_mem),
            data_mem: Banker::new([0; 64]),
            devices: mapped_devices,
        }
    }

    pub fn tick(&mut self) -> Halted {
        let inst = self.fetch();
        self.process(inst)
    }

    pub fn fetch(&self) -> Instruction {
        Instruction::decode([
            self.inst_mem[self.reg_zero as usize],
            self.inst_mem[(self.reg_zero + 1) as usize],
            self.inst_mem[(self.reg_zero + 2) as usize],
        ])
    }

    fn process(&mut self, inst: Instruction) -> Halted {
        use crate::OpCode::*;
        let result: u8 = match inst.code {
            NoOp => 0,
            And => {
                let ((data1, result1), (data2, result2)) = self.load_double(&inst.arg1, &inst.arg2);
                let result3 = self.push(&inst.arg1, data1 & data2);
                result1 | result2 << 1 | result3 << 2
            }
            Or => {
                let ((data1, result1), (data2, result2)) = self.load_double(&inst.arg1, &inst.arg2);
                let result3 = self.push(&inst.arg1, data1 | data2);
                result1 | result2 << 1 | result3 << 2
            }
            Not => {
                let (data1, result1) = self.load(&inst.arg1);
                let result3 = self.push(&inst.arg1, !data1);
                result1 | result3 << 2
            }
            Add => {
                let ((data1, result1), (data2, result2)) =
                    self.load_double_signed(&inst.arg1, &inst.arg2, &inst.options);
                let result3 = self.push(&inst.arg1, (data1 + data2) as u8);
                result1 | result2 << 1 | result3 << 2
            }
            Sub => {
                let ((data1, result1), (data2, result2)) =
                    self.load_double_signed(&inst.arg1, &inst.arg2, &inst.options);
                let result3 = self.push(&inst.arg1, (data1 - data2) as u8);
                result1 | result2 << 1 | result3 << 2
            }
            Mul => {
                let ((data1, result1), (data2, result2)) =
                    self.load_double_signed(&inst.arg1, &inst.arg2, &inst.options);
                let result3 = self.push(&inst.arg1, (data1 * data2) as u8);
                result1 | result2 << 1 | result3 << 2
            }
            Div => {
                let ((data1, result1), (data2, result2)) =
                    self.load_double_signed(&inst.arg1, &inst.arg2, &inst.options);
                let result3 = self.push(&inst.arg1, (data1 / data2) as u8);
                result1 | result2 << 1 | result3 << 2
            }
            SL => {
                let (data1, result1) = self.load(&inst.arg1);
                let result3 = self.push(&inst.arg1, data1 << 1);
                result1 | result3 << 2
            }
            SR => {
                let (data1, result1) = self.load(&inst.arg1);
                let result3 = self.push(&inst.arg1, data1 >> 1);
                result1 | result3 << 2
            }
            RL => {
                let (data1, result1) = self.load(&inst.arg1);
                let result3 = self.push(&inst.arg1, data1.rotate_left(1));
                result1 | result3 << 2
            }
            RR => {
                let (data1, result1) = self.load(&inst.arg1);
                let result3 = self.push(&inst.arg1, data1.rotate_right(1));
                result1 | result3 << 2
            }
            Copy => {
                let (data1, result1) = self.load(&inst.arg1);
                let result3 = self.push(&inst.arg2, data1);
                result1 | result3 << 2
            }
            CompEq => {
                let ((data1, result1), (data2, result2)) = self.load_double_signed(&inst.arg1, &inst.arg2, &inst.options);
                if data1 != data2 {
                    self.reg_zero += 3;
                }
                result1 | result2 << 1
            }
            CompGt => {
                let ((data1, result1), (data2, result2)) = self.load_double_signed(&inst.arg1, &inst.arg2, &inst.options);
                if data1 <= data2 {
                    self.reg_zero += 3;
                }
                result1 | result2 << 1
            }
            CompLt => {
                let ((data1, result1), (data2, result2)) = self.load_double_signed(&inst.arg1, &inst.arg2, &inst.options);
                if data1 >= data2 {
                    self.reg_zero += 3;
                }
                result1 | result2 << 1
            }
        };
        if inst.options.halt_on_error() && result != 0 {
            return Halted::Halted;
        }

        self.reg_zero += 3;
        Halted::Running
    }

    fn load_double(&mut self, addr1: &Symbol, addr2: &Symbol) -> ((u8, u8), (u8, u8)) {
        (self.load(addr1), self.load(addr2))
    }

    fn load_double_signed(
        &mut self,
        addr1: &Symbol,
        addr2: &Symbol,
        options: &OpOptions,
    ) -> ((i16, u8), (i16, u8)) {
        let (data1, result1) = self.load(addr1);
        let (data2, result2) = self.load(addr2);
        let mut data1_signed = data1 as i16;
        let mut data2_signed = data2 as i16;
        if options.arg1_signed() {
            data1_signed = i8::from_be_bytes([data1]) as i16;
        }
        if options.arg2_signed() {
            data2_signed = i8::from_be_bytes([data2]) as i16;
        }
        ((data1_signed, result1), (data2_signed, result2))
    }

    fn load(&mut self, addr: &Symbol) -> (u8, u8) {
        match addr.address() {
            0 => (self.reg_zero, 0),
            1..=127 => (self.inst_mem[addr.address() as usize], 0),
            128..=191 => (self.data_mem[(addr.address() - 128) as usize], 0),
            192 => (self.inst_mem.pointer, 0),
            193 => (self.data_mem.pointer, 0),
            _ => (0, 1),
        }
    }

    fn push(&mut self, addr: &Symbol, data: u8) -> u8 {
        println!("From push(): addr:{}, data:{}", addr.address(), data);
        match addr.address() {
            0 => self.reg_zero = data,
            1..=127 => self.inst_mem[addr.address() as usize] = data,
            128..=191 => self.data_mem[(addr.address() - 128) as usize] = data,
            192 => self.inst_mem.pointer = data,
            193 => self.data_mem.pointer = data,
            _ => return 1,
        }
        0
    }
}
