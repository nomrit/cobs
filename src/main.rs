use std::io;
use std::num::Wrapping;
use std::env;

fn usage(program: &str) {
    let head = format!("usage : {} <in-file> <out-file>",program);
    println!("{}",head);
}

fn encode(src: & [u8], dst:& mut [u8]) -> usize {
    for i in 0..src.len() {
        dst[i] = (Wrapping(src[i])+Wrapping(1u8)).0
    }
    return src.len()
}

fn main() -> io::Result<()> {

    if env::args().count()!=3 {
        let args: Vec<String> = env::args().collect();

        usage(&args[0]);
        std::process::exit(1);
    }

    let mut src = [0;256];
    let mut dst = [0;256];

    for i in 0..256 {
        src[i] = i as u8;
    }
    
    let r = encode(&src,&mut dst);

    assert_eq!(r,256);

    for i in 0..256 {
        println!("i={}, src={:<02x}, dst={:<02x}",i,src[i],dst[i])
    }

    std::process::exit(0);
}
