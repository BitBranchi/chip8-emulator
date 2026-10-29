# chip8-emulator

A CHIP-8 emulator written in Rust, using SDL2 for rendering.

## About

I built this as a learning project to understand how a computer works at a low level, by writing the fetch-decode-execute loop myself, and to learn Rust through a real project instead of isolated exercises.

It's an early version: it can already run simple ROMs such as the CHIP-8 logo test.

## Status

**Implemented**
- [x] Fetch / decode / execute loop
- [x] 4 KB memory, 16 registers (V0-VF), index register, program counter
- [x] 64x32 display, drawn with XOR and collision detection (VF)
- [x] Opcodes: `00E0`, `1NNN`, `6XNN`, `7XNN`, `ANNN`, `DXYN`

**Planned**
- [ ] Remaining opcodes (subroutines, conditionals, arithmetic, etc.)
- [ ] Keyboard input
- [ ] Delay and sound timers
- [ ] Built-in font
- [ ] Cycle/frame timing

Unknown opcodes currently cause a panic that prints the opcode, which makes it easy to see what to implement next.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (stable)
- [SDL2](https://github.com/Rust-SDL2/rust-sdl2#requirements) development libraries

## Build and run

```sh
git clone https://github.com/BitBranchi/chip8-emulator/
cd chip8-emulator
cargo run --release -- path/to/rom.ch8
```

Press `Esc` or close the window to quit.

## What I learned

- How a CPU fetches and decodes instructions, and how memory, registers, and display work together
- Rust basics: structs and `impl`, pattern matching on nibbles, `Result` and `?`, and integer wrapping
- Using SDL2 from Rust for windowing, events, and drawing

## Resources

- [Timendus CHIP-8 test suite](https://github.com/Timendus/chip8-test-suite), the ROMs I use for testing
