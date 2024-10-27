# CHIP-8/SUPER-CHIP Assembler

This is a simple assembly -> byte code compiler for a mostly (see differences [at the bottom](#contributing)) standard CHIP-8 or SUPER-CHIP emulator.

## **Note**:

This project was an experiment and learning resource first. I am new to Rust and understand that this is incredibly far from perfect. I'm not really doing much more than throwing errors, and it often compiles code based on what it thinks you wanted instead of just raising exceptions.

If you are looking for an accurate CHIP-8 assembler, I recommend looking for another project, but if you want to let me know the things I did wrong or could do better, please do!

## Resources

This was written entirely by me in Rust, but is based largely on [Cowgod's Chip-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#Fx75).

I also build the emulator I tested with initially following the fantastic book [from Aquova](https://github.com/aquova/chip8-book).

## Features

- Assembles CHIP-8 assembly code into bytecode
- Supports all standard CHIP-8 instructions
- Include other assembly files with `include`
  - All includes are added to the end of the primary file
  - Included files either need to be adjacent to the main ASM file, or in the working directory

## Instructions

It is pretty simple to compile code:

```sh
cargo run 'in/path/to/asm/file' 'out/path/to/bin' [decimal-offset]
```

The offset is (512) 0x200 by default, which is where the compiler assumes that the binary file will be loaded into the CHIP-8 emulator.

## Examples

Below is just some examples of some assembly code with some directives this compiler can handle.

```assembly
define val #F

start:
    db 0xf 0xF val ; reserve three bytes of space (0xFFF)
    db %00001000 ; reserve one byte of space (0x8)

loop:
    JP loop ; infinite loop

string: text "This is a string of text" ; Text is stored as null terminated ASCII lists
```

## Differences from standard CHIP-8

This is designed to compile any standard CHIP-8 or SUPER-CHIP instructions (check Cowgod's reference). However, I added interpretation for two additional commands to read and write the `I` register from two `V` registers:

- 5xy1 - `LD Vx, Vy, I` | Read higher and lower bit of `I` into Vx and Vy
- 5xy2 - `LD I, Vx, Vy` | Write Vx and Vy into the higher and lower bit of `I`

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
