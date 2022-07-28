use std::env;
use std::fs::File;
use std::io::prelude::*;

const BYTES_PER_LINE: usize = 16;

fn main() -> std::io::Result<()> {
    let arg1 = env::args().nth(1);
    let fname = arg1.expect("usage: fview FILENAME");
    let mut fp = File::open(&fname).expect("Unable to open file.");

    let mut pos_in_input = 0;
    let mut buff = [0; BYTES_PER_LINE];

    while let Ok(_) = fp.read_exact(&mut buff) {
        print!("[0x{:08x}] ", pos_in_input);
        for byte in buff {
            print!("{:02x} ", byte);
        }            
        println!();
        pos_in_input += BYTES_PER_LINE;
    }

    Ok(())
}
