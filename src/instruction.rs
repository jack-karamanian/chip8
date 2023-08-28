use crate::cpu::Register;

pub enum Instruction {
    Cls,
    Ret,
    Jmp(u16),
    Call(u16),
    SkipInstructionEqual(Register, u8),
    SkipInstructionNotEqual(Register, u8),
    LoadConstant(Register, u8),
    Add(Register, u8),
    LoadRegister(Register, Register),
    OrRegister(Register, Register),
    AndRegister(Register, Register),
    XorRegister(Register, Register),
    AddRegister(Register, Register),
    SubRegister(Register, Register),
    Shr(Register),
    Subn(Register, Register),
    Shl(Register),
    SkipInstructionRegisterEqual(Register, Register),
    SkipInstructionRegisterNotEqual(Register, Register),
    LoadIndex(u16),
    JumpV0(u16),
    Random(Register, u8),
    Draw(Register, Register, u8),
    SkipIfPressed(Register),
    SkipIfNotPressed(Register),
    LoadDelayTimer(Register),
    WaitForKeyPress(Register),
    StoreDelayTimer(Register),
    StoreSoundTimer(Register),
    AddIndex(Register),
    LoadSpriteIndex(Register),
    StoreBcd(Register),
    StoreRegisters(Register /* last register */),
    LoadRegisters(Register /* last register */),
    Invalid,
}

fn x_value(value: u16) -> Register {
    Register::from_index(((value >> 8) & 0b1111) as u8)
}

fn y_value(value: u16) -> Register {
    Register::from_index(((value >> 4) & 0b1111) as u8)
}

impl Instruction {
    pub fn decode(value: u16) -> Instruction {
        if value == 0x00E0 {
            return Instruction::Cls;
        }
        if value == 0x00EE {
            return Instruction::Ret;
        }
        match value & 0xF000 {
            0x1000 => return Instruction::Jmp(value & 0x0FFF),
            0x2000 => return Instruction::Call(value & 0x0FFF),
            0x3000 => {
                return Instruction::SkipInstructionEqual(x_value(value), (value & 0x00FF) as u8)
            }
            0x4000 => {
                return Instruction::SkipInstructionNotEqual(x_value(value), (value & 0x00FF) as u8)
            }
            0x6000 => return Instruction::LoadConstant(x_value(value), (value & 0xFF) as u8),
            0x7000 => return Instruction::Add(x_value(value), (value & 0xFF) as u8),
            0xA000 => return Instruction::LoadIndex(value & 0xFFF),
            0xB000 => return Instruction::JumpV0(value & 0xFFF),
            0xC000 => return Instruction::Random(x_value(value), (value & 0xFF) as u8),
            0xD000 => {
                return Instruction::Draw(x_value(value), y_value(value), (value & 0xF) as u8)
            }

            _ => {}
        }

        if value & 0xF00F == 0x5000 {
            return Instruction::SkipInstructionRegisterEqual(x_value(value), y_value(value));
        }

        if value & 0xF00F == 0x8000 {
            return Instruction::LoadRegister(x_value(value), y_value(value));
        }
        if value & 0xF00F == 0x8001 {
            return Instruction::OrRegister(x_value(value), y_value(value));
        }
        if value & 0xF00F == 0x8002 {
            return Instruction::AndRegister(x_value(value), y_value(value));
        }
        if value & 0xF00F == 0x8003 {
            return Instruction::XorRegister(x_value(value), y_value(value));
        }
        if value & 0xF00F == 0x8004 {
            return Instruction::AddRegister(x_value(value), y_value(value));
        }
        if value & 0xF00F == 0x8005 {
            return Instruction::SubRegister(x_value(value), y_value(value));
        }
        if value & 0xF00F == 0x8006 {
            return Instruction::Shr(x_value(value));
        }
        if value & 0xF00F == 0x8007 {
            return Instruction::Subn(x_value(value), y_value(value));
        }
        if value & 0xF00F == 0x800E {
            return Instruction::Shl(x_value(value));
        }
        if value & 0xF00F == 0x9000 {
            return Instruction::SkipInstructionRegisterNotEqual(x_value(value), y_value(value));
        }
        if value & 0xF0FF == 0xE09E {
            return Instruction::SkipIfPressed(x_value(value));
        }
        if value & 0xF0FF == 0xE0A1 {
            return Instruction::SkipIfNotPressed(x_value(value));
        }
        if value & 0xF0FF == 0xF007 {
            return Instruction::LoadDelayTimer(x_value(value));
        }
        if value & 0xF0FF == 0xF00A {
            return Instruction::WaitForKeyPress(x_value(value));
        }
        if value & 0xF0FF == 0xF015 {
            return Instruction::StoreDelayTimer(x_value(value));
        }
        if value & 0xF0FF == 0xF018 {
            return Instruction::StoreSoundTimer(x_value(value));
        }
        if value & 0xF0FF == 0xF01E {
            return Instruction::AddIndex(x_value(value));
        }
        if value & 0xF0FF == 0xF029 {
            return Instruction::LoadSpriteIndex(x_value(value));
        }
        if value & 0xF0FF == 0xF033 {
            return Instruction::StoreBcd(x_value(value));
        }
        if value & 0xF0FF == 0xF055 {
            return Instruction::StoreRegisters(x_value(value));
        }
        if value & 0xF0FF == 0xF065 {
            return Instruction::LoadRegisters(x_value(value));
        }

        Instruction::Invalid
    }
}
