use crate::mmu::Mmu;

pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

pub struct Graphics {
    display: [[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
}

impl Graphics {
    pub fn new() -> Graphics {
        Graphics {
            display: [[0; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
        }
    }

    pub fn to_rgba(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(DISPLAY_WIDTH * DISPLAY_HEIGHT * 4);
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                data.push(if self.display[y][x] > 0 { 128 as u8 } else { 0 });
                data.push(0);
                data.push(0);
                data.push(255);
            }
        }
        data
    }

    pub fn clear(&mut self) {
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                self.display[y][x] = 0;
            }
        }
    }

    pub fn draw(&mut self, index: usize, num_bytes: usize, x: usize, y: usize, mmu: &Mmu) -> bool {
        let mut overwrote_pixel = false;

        let x_coord = x % DISPLAY_WIDTH;
        let y_coord = y % DISPLAY_HEIGHT;

        for row in 0..num_bytes {
            let bits = mmu.read8((index + row) as u16);
            let cy = (y_coord + row) % DISPLAY_HEIGHT;

            for col in 0..8 {
                let cx = (x_coord + col) % DISPLAY_WIDTH;
                let current_col = self.display[cy][cx];
                let col = bits & (0x01 << 7 - col);

                if col > 0 {
                    if current_col > 0 {
                        self.display[cy][cx] = 0;
                        overwrote_pixel = true
                    } else {
                        self.display[cy][cx] = 1;
                    }
                }

                if cx == DISPLAY_WIDTH - 1 {
                    break;
                }
            }
            if cy == DISPLAY_HEIGHT - 1 {
                break;
            }
        }
        overwrote_pixel
    }
}
