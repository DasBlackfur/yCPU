use crate::{banking::Banker, devices::Device, Halted, Instruction};

pub struct CPU {
    pub reg_zero: u8,
    pub inst_mem: Banker<[u8; 127]>,
    pub data_mem: Banker<[u8; 64]>,
    pub devices: Vec<Box<dyn Device>>,
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
        Instruction::from_3bytes([
            self.inst_mem[self.reg_zero as usize],
            self.inst_mem[(self.reg_zero + 1) as usize],
            self.inst_mem[(self.reg_zero + 2) as usize],
        ])
    }

    fn process(&mut self, inst: Instruction) -> Halted {
        match inst {
            Instruction::NoOp(_, _, _, _) => (),
            Instruction::And(_, _, _, _, arg1, arg2) => {
                let data1 = self.load(arg1);
                let data2 = self.load(arg2);
                self.push(arg1, data1 & data2);
            }
            Instruction::Or(_, _, _, _, arg1, arg2) => {
                let data1 = self.load(arg1);
                let data2 = self.load(arg2);
                self.push(arg1, data1 | data2);
            }
            Instruction::Not(_, _, _, _, arg1) => {
                let data1 = self.load(arg1);
                self.push(arg1, !data1);
            }
            Instruction::Add(_, _, sign1, sign2, arg1, arg2) => {
                match (sign1, sign2) {
                    (true, true) => {
                        let data1 = i8::from_be_bytes([self.load(arg1)]);
                        let data2 = i8::from_be_bytes([self.load(arg2)]);
                        self.push(arg1, (data1 + data2) as u8);
                    }
                    (true, false) => {
                        let data1 = i8::from_be_bytes([self.load(arg1)]);
                        let data2 = self.load(arg2);
                        self.push(arg1, (data1 as i16 + data2 as i16) as u8);
                    }
                    (false, true) => {
                        let data1 = self.load(arg1);
                        let data2 = i8::from_be_bytes([self.load(arg2)]);
                        self.push(arg1, (data1 as i16 + data2 as i16) as u8);
                    }
                    (false, false) => {
                        let data1 = self.load(arg1);
                        let data2 = self.load(arg2);
                        self.push(arg1, data1 + data2);
                    }
                };
            }
            Instruction::Sub(_, _, sign1, sign2, arg1, arg2) => match (sign1, sign2) {
                (true, true) => {
                    let data1 = i8::from_be_bytes([self.load(arg1)]);
                    let data2 = i8::from_be_bytes([self.load(arg2)]);
                    self.push(arg1, (data1 - data2) as u8);
                }
                (true, false) => {
                    let data1 = i8::from_be_bytes([self.load(arg1)]);
                    let data2 = self.load(arg2);
                    self.push(arg1, (data1 as i16 - data2 as i16) as u8);
                }
                (false, true) => {
                    let data1 = self.load(arg1);
                    let data2 = i8::from_be_bytes([self.load(arg2)]);
                    self.push(arg1, (data1 as i16 - data2 as i16) as u8);
                }
                (false, false) => {
                    let data1 = self.load(arg1);
                    let data2 = self.load(arg2);
                    self.push(arg1, data1 - data2);
                }
            },
            Instruction::Mul(_, _, sign1, sign2, arg1, arg2) => match (sign1, sign2) {
                (true, true) => {
                    let data1 = i8::from_be_bytes([self.load(arg1)]);
                    let data2 = i8::from_be_bytes([self.load(arg2)]);
                    self.push(arg1, (data1 * data2) as u8);
                }
                (true, false) => {
                    let data1 = i8::from_be_bytes([self.load(arg1)]);
                    let data2 = self.load(arg2);
                    self.push(arg1, (data1 as i16 * data2 as i16) as u8);
                }
                (false, true) => {
                    let data1 = self.load(arg1);
                    let data2 = i8::from_be_bytes([self.load(arg2)]);
                    self.push(arg1, (data1 as i16 * data2 as i16) as u8);
                }
                (false, false) => {
                    let data1 = self.load(arg1);
                    let data2 = self.load(arg2);
                    self.push(arg1, data1 * data2);
                }
            },
            Instruction::Div(_, _, sign1, sign2, arg1, arg2) => match (sign1, sign2) {
                (true, true) => {
                    let data1 = i8::from_be_bytes([self.load(arg1)]);
                    let data2 = i8::from_be_bytes([self.load(arg2)]);
                    self.push(arg1, (data1 / data2) as u8);
                }
                (true, false) => {
                    let data1 = i8::from_be_bytes([self.load(arg1)]);
                    let data2 = self.load(arg2);
                    self.push(arg1, (data1 as i16 / data2 as i16) as u8);
                }
                (false, true) => {
                    let data1 = self.load(arg1);
                    let data2 = i8::from_be_bytes([self.load(arg2)]);
                    self.push(arg1, (data1 as i16 / data2 as i16) as u8);
                }
                (false, false) => {
                    let data1 = self.load(arg1);
                    let data2 = self.load(arg2);
                    self.push(arg1, data1 / data2);
                }
            },
            Instruction::SL(_, _, _, _, arg1) => {
                let data1 = self.load(arg1);
                self.push(arg1, data1 << 1)
            }
            Instruction::SR(_, _, _, _, arg1) => {
                let data1 = self.load(arg1);
                self.push(arg1, data1 >> 1)
            }
            Instruction::RL(_, _, _, _, arg1) => {
                let data1 = self.load(arg1);
                self.push(arg1, u8::rotate_left(data1, 1));
            }
            Instruction::RR(_, _, _, _, arg1) => {
                let data1 = self.load(arg1);
                self.push(arg1, u8::rotate_right(data1, 1));
            }
            Instruction::Copy(_, _, _, _, arg1, arg2) => {
                let data1 = self.load(arg1);
                self.push(arg2, data1);
            }
            Instruction::CompEq(_, _, _, _, arg1, arg2) => {
                let data1 = self.load(arg1);
                let data2 = self.load(arg2);
                if data1 != data2 {
                    self.reg_zero += 3;
                }
            }
            Instruction::CompGt(_, _, _, _, arg1, arg2) => {
                let data1 = self.load(arg1);
                let data2 = self.load(arg2);
                if data1 <= data2 {
                    self.reg_zero += 3;
                }
            }
            Instruction::CompLt(_, _, _, _, arg1, arg2) => {
                let data1 = self.load(arg1);
                let data2 = self.load(arg2);
                if data1 >= data2 {
                    self.reg_zero += 3;
                }
            }
        };
        self.reg_zero += 3;
        Halted::Running
    }

    fn load(&mut self, addr: u8) -> u8 {
        match addr {
            0 => self.reg_zero,
            1..=127 => self.inst_mem[addr as usize],
            128..=191 => self.data_mem[(addr - 128) as usize],
            192 => self.inst_mem.pointer,
            193 => self.data_mem.pointer,
            _ => panic!("Invalid address: {}", addr),
        }
    }

    fn push(&mut self, addr: u8, data: u8) {
        println!("From push(): addr:{}, data:{}", addr, data);
        match addr {
            0 => self.reg_zero = data,
            1..=127 => self.inst_mem[addr as usize] = data,
            128..=191 => self.data_mem[(addr - 128) as usize] = data,
            192 => self.inst_mem.pointer = data,
            193 => self.data_mem.pointer = data,
            _ => panic!("Invalid address: {}", addr),
        }
    }
}
