use std::env;
use std::io::Write;

mod asm;
use asm::generate_full_asm;

mod instructions;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: cargo run 'path/to/asm' 'path/to/out' [offset]");
        return;
    }

    let offset = if args.len() > 3 {
        args[3].parse().unwrap()
    } else {
        0x200
    };
    let mut full_asm = generate_full_asm(&args[1], offset);

    let bytes = full_asm.to_bytes();

    // Write to file
    let mut file = std::fs::File::create(&args[2]).unwrap();
    file.write_all(&bytes).unwrap();
}
