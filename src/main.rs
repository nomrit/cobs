use std::io;
use std::env;
mod cobs;
use cobs::*;

fn usage(program: &str) {
    let head = format!("usage : {} <in-file> <out-file>",program);
    println!("{}",head);
    std::process::exit(1);
}

fn cobstest(src:Vec<u8>) -> bool {
    let v1 = encode(&src,src.len());
    println!("{:?}",v1);
    let v2 = cobs::decode(v1);
    println!("{:?}",v2);

    assert_eq!(src,v2);
    return true;
}

fn main() -> io::Result<()> {
    let mut args = env::args();

    let cmd = args.next().unwrap();
    if args.len() !=2 {
        usage(&cmd);
    }
    let infile = args.next().unwrap();
    let outfile = args.next().unwrap();

    println!("{}",cmd);
    println!("{}",infile);
    println!("{}",outfile);

    {
        let src:Vec<u8>=vec![0];
        println!("*****TEST1*****");
        println!("SRC={:?}",src);

        cobstest(src);
    }
    {
        let mut src:Vec<u8>=Vec::new();

        for i in 1..0x100 {
            src.push(i as u8);
        }
        println!("*****TEST2*****");
        println!("SRC={:?}",src);
    
        cobstest(src);
    }
    {
        let mut src:Vec<u8>=Vec::new();

        for i in 1..0xff {
            src.push(i as u8);
        }
        println!("*****TEST3*****");
        println!("SRC={:?}",src);
    
        cobstest(src);
    }
    {
        let mut src:Vec<u8> = Vec::new();

        println!("*****TEST4*****");
        for i in 2..0x100 {
            src.push(i as u8);
        }
        src.push(0 as u8);
        println!("SRC={:?}",src);

        cobstest(src);
    }
    {
        let src:Vec<u8>=vec![1,0,0,0];
        println!("*****TEST5*****");
        println!("SRC={:?}",src);

        cobstest(src);
    }
    {
        let src:Vec<u8>=vec![0,0];
        println!("*****TEST6*****");
        println!("SRC={:?}",src);

        cobstest(src);
    }
    std::process::exit(0);
}
