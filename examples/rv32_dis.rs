
use machine::isa::*;
use machine::isa::rv32i::*;

use std::fs::File;
use std::io::Read;

fn main() { 

    let mut f = File::open("tests/rv32i/test.bin").unwrap();
    let len = f.metadata().unwrap().len() as usize;
    let mut buf = vec![0u8; len];
    f.read(&mut buf).unwrap();

    let instrs = unsafe { 
        std::slice::from_raw_parts(buf.as_ptr() as *const u32, len/4)
    };

    for inst in instrs {
        let res = Rv32::decode(*inst);
        println!("{}", res);
    }

}
