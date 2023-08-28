pub struct Mmu {
    memory: [u8; 4096],
    stack: [u16; 1024],
    sp: usize,
}

const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

impl Mmu {
    pub fn new() -> Mmu {
        let mut mmu = Mmu {
            memory: [0; 4096],
            stack: [0; 1024],
            sp: 1024,
        };

        for (i, &value) in FONT.iter().enumerate() {
            mmu.memory[i] = value;
        }

        mmu
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        for (i, &element) in rom.iter().enumerate() {
            self.memory[0x200 + i] = element;
        }
    }

    pub fn push_stack(&mut self, value: u16) {
        self.sp -= 1;
        self.stack[self.sp] = value;
    }

    pub fn pop_stack(&mut self) -> u16 {
        let value = self.stack[self.sp];
        self.sp += 1;
        value
    }

    pub fn write8(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    pub fn read8(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write16(&mut self, address: u16, value: u16) {
        self.write8(address, ((value & 0xff00) >> 8) as u8);
        self.write8(address + 1, ((value & 0xff) >> 8) as u8);
    }

    pub fn read16(&self, address: u16) -> u16 {
        ((self.read8(address) as u16) << 8) | self.read8(address + 1) as u16
    }
}
