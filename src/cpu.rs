use crate::graphics::Graphics;
use crate::instruction::Instruction;
use crate::mmu::Mmu;

pub struct Cpu {
    registers: [u8; 16],
    pc: u16,
    index: u16,
}

#[derive(Copy, Clone)]
pub enum Register {
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    VA,
    VB,
    VC,
    VD,
    VE,
    VF,
}

impl Register {
    fn to_index(&self) -> usize {
        match self {
            Register::V0 => 0,
            Register::V1 => 1,
            Register::V2 => 2,
            Register::V3 => 3,
            Register::V4 => 4,
            Register::V5 => 5,
            Register::V6 => 6,
            Register::V7 => 7,
            Register::V8 => 8,
            Register::V9 => 9,
            Register::VA => 10,
            Register::VB => 11,
            Register::VC => 12,
            Register::VD => 13,
            Register::VE => 14,
            Register::VF => 15,
        }
    }

    pub fn from_index(value: u8) -> Register {
        match value {
            0 => Register::V0,
            1 => Register::V1,
            2 => Register::V2,
            3 => Register::V3,
            4 => Register::V4,
            5 => Register::V5,
            6 => Register::V6,
            7 => Register::V7,
            8 => Register::V8,
            9 => Register::V9,
            10 => Register::VA,
            11 => Register::VB,
            12 => Register::VC,
            13 => Register::VD,
            14 => Register::VE,
            15 => Register::VF,
            _ => panic!("Invalid register"),
        }
    }
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: [0; 16],
            pc: 0x200,
            index: 0,
        }
    }

    pub fn reg(&self, register: Register) -> u8 {
        self.registers[register.to_index()]
    }

    pub fn set_reg(&mut self, register: Register, value: u8) {
        self.registers[register.to_index()] = value;
    }

    pub fn step(&mut self, mmu: &mut Mmu, graphics: &mut Graphics) {
        let instruction_value = mmu.read16(self.pc);
        self.pc += 2;
        match Instruction::decode(instruction_value) {
            Instruction::Cls => graphics.clear(),
            Instruction::Ret => self.pc = mmu.pop_stack(),
            Instruction::Jmp(addr) => self.pc = addr,
            Instruction::Call(addr) => {
                mmu.push_stack(self.pc);
                self.pc = addr;
            }
            Instruction::SkipInstructionEqual(register, constant) => {
                if self.reg(register) == constant {
                    self.pc += 2;
                }
            }
            Instruction::SkipInstructionNotEqual(register, constant) => {
                if self.reg(register) != constant {
                    self.pc += 2;
                }
            }
            Instruction::LoadConstant(register, constant) => self.set_reg(register, constant),
            Instruction::Add(register, constant) => {
                self.set_reg(register, self.reg(register) + constant);
            }
            Instruction::LoadRegister(x, y) => self.set_reg(x, self.reg(y)),
            Instruction::OrRegister(x, y) => self.set_reg(x, self.reg(x) | self.reg(y)),
            Instruction::AndRegister(x, y) => self.set_reg(x, self.reg(x) & self.reg(y)),
            Instruction::XorRegister(x, y) => self.set_reg(x, self.reg(x) ^ self.reg(y)),
            Instruction::AddRegister(x, y) => {
                let x_value = self.reg(x);
                let y_value = self.reg(y);
                self.set_reg(
                    Register::VF,
                    if x_value as u16 + y_value as u16 > 0xFF {
                        1
                    } else {
                        0
                    },
                );
                self.set_reg(x, x_value + y_value);
            }
            Instruction::SubRegister(x, y) => {
                let x_value = self.reg(x);
                let y_value = self.reg(y);
                self.set_reg(Register::VF, if x_value > y_value { 1 } else { 0 });
                self.set_reg(x, x_value - y_value);
            }
            Instruction::Shr(x) => {
                let x_value = self.reg(x);

                self.set_reg(Register::VF, x_value & 0x01);
                self.set_reg(x, x_value >> 1);
            }
            Instruction::Subn(x, y) => {
                let x_value = self.reg(x);
                let y_value = self.reg(y);
                self.set_reg(Register::VF, if y_value > x_value { 1 } else { 0 });
                self.set_reg(x, y_value - x_value);
            }
            Instruction::Shl(x) => {
                let x_value = self.reg(x);

                self.set_reg(Register::VF, x_value & 0x80);
                self.set_reg(x, x_value << 1);
            }
            Instruction::SkipInstructionRegisterEqual(x, y) => {
                if self.reg(x) == self.reg(y) {
                    self.pc += 2;
                }
            }
            Instruction::SkipInstructionRegisterNotEqual(x, y) => {
                if self.reg(x) != self.reg(y) {
                    self.pc += 2;
                }
            }
            Instruction::LoadIndex(constant) => self.index = constant,
            Instruction::JumpV0(constant) => self.pc = self.reg(Register::V0) as u16 + constant,
            Instruction::Random(x, mask) => {
                self.set_reg(x, rand::random::<u8>() & mask);
            }
            Instruction::Draw(x, y, num_bytes) => {
                let x_coord = self.reg(x);
                let y_coord = self.reg(y);
                if graphics.draw(
                    self.index as usize,
                    num_bytes as usize,
                    x_coord as usize,
                    y_coord as usize,
                    mmu,
                ) {
                    self.set_reg(Register::VF, 1);
                }
            }
            Instruction::SkipIfPressed(_) => todo!(),
            Instruction::SkipIfNotPressed(_) => todo!(),
            Instruction::LoadDelayTimer(_) => todo!(),
            Instruction::WaitForKeyPress(_) => todo!(),
            Instruction::StoreDelayTimer(_) => todo!(),
            Instruction::StoreSoundTimer(_) => todo!(),
            Instruction::AddIndex(_) => todo!(),
            Instruction::LoadSpriteIndex(_) => todo!(),
            Instruction::StoreBcd(_) => todo!(),
            Instruction::StoreRegisters(_) => todo!(),
            Instruction::LoadRegisters(_) => todo!(),
            Instruction::Invalid => todo!(),
        }
    }
}
