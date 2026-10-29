extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

const START: usize = 0x200;

struct Cpu {
    memory: [u8; 4096],
    v: [u8; 16], // registers V0..VF
    i: u16,      // index register
    pc: u16,     // program counter
    stack: [u16; 16],
    sp: usize, // stack pointer
    display: [bool; 64 * 32],
}

impl Cpu {
    fn new() -> Self {
        Cpu {
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: START as u16,
            stack: [0; 16],
            sp: 0,
            display: [false; 64 * 32],
        }
    }

    fn fetch(&mut self) -> u16 {
        let b1 = self.memory[self.pc as usize] as u16;
        let b2 = self.memory[(self.pc + 1) as usize] as u16;

        let opcode = (b1 << 8) | b2;

        self.pc += 2;

        opcode
    }

    fn load_rom(&mut self, path: &str) -> Result<(), std::io::Error> {
        let bytes = std::fs::read(path)?;
        self.memory[START..(START + bytes.len())].copy_from_slice(&bytes);
        Ok(())
    }

    fn execute(&mut self, opcode: u16) {
        let nibbles = (
            (opcode & 0xF000) >> 12,
            (opcode & 0x0F00) >> 8,
            (opcode & 0x00F0) >> 4,
            (opcode & 0x000F),
        );
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        match nibbles {
            (0x0, 0x0, 0xE, 0x0) => {
                self.display.fill(false);
            }
            (0x1, _, _, _) => {
                self.pc = nnn;
            }
            (0x6, _, _, _) => {
                self.v[x] = nn;
            }
            (0x7, _, _, _) => {
                self.v[x] = self.v[x].wrapping_add(nn);
            }
            (0xA, _, _, _) => {
                self.i = nnn;
            }
            (0xD, _, _, _) => {
                let y = ((opcode & 0x00F0) >> 4) as usize;
                let n = (opcode & 0x000F) as usize;
                let x_pos = self.v[x] as usize % 64;
                let y_pos = self.v[y] as usize % 32;

                self.v[0xF] = 0;
                for row in 0..n {
                    let byte = self.memory[self.i as usize + row];
                    for col in 0..8 {
                        let bit = (byte >> (7 - col)) & 1;
                        let px = x_pos + col;
                        let py = y_pos + row;
                        if px >= 64 || py >= 32 {
                            continue;
                        }
                        if bit == 1 {
                            let idx = py * 64 + px;
                            if self.display[idx] {
                                self.v[0xF] = 1;
                            }
                            self.display[idx] ^= true;
                        }
                    }
                }
            }
            _ => panic!("unknown opcode : {:#06x}", opcode),
        }
    }
    fn print_display(&self) {
        for row in 0..32 {
            let line: String = (0..64)
                .map(|col| {
                    if self.display[row * 64 + col] {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect();
            println!("{line}");
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("CHIP8", 640, 320)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Variables
    let mut args = std::env::args();
    let arg = args.nth(1).unwrap();
    let mut cpu = Cpu::new();
    cpu.load_rom(&arg.to_string())?;

    let mut rect = Rect::new(0, 0, 10, 10);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        let opcode = cpu.fetch();
        cpu.execute(opcode);

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(0, 255, 0));

        // Render CHIP-8 display
        for (i, &pixel) in cpu.display.iter().enumerate() {
            if pixel {
                let x = (i % 64) as i32;
                let y = (i / 64) as i32;

                // Scale up each CHIP-8 pixel by 10x to fit the 640x320 window
                rect.x = x * 10;
                rect.y = y * 10;

                canvas.fill_rect(rect)?;
            }
        }

        canvas.present();
    }
    Ok(())
}
