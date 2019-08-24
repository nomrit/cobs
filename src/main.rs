use std::io;
// use std::io::*;

/*
 struct info {
     u8 type;
     u8 office;
     u8 platform;
     u32 time;
     u8 bandwidth;
     u8 channel;
     u32 lat,lon; u16 alt;
     u16 roll,pitch,yaw;
     u8 cameratype;
     u16 pan,tilt,roll,fov;
     u16 north,east,down;
     u16 lat_s,lon_s; u16 alt_s;
}; */

/*
fn usage(program: &str, opt: Options) {
    let head = format!("usage {} [options] <file-name>",program);
    print!("{}",opt.usage(&head));
} */

fn encode(src: & [u8], dst:& mut [u8]) -> usize {
    for i in 0..src.len() {
        dst[i] = src[i];
    }
    return src.len()
}

fn main() -> io::Result<()> {
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

    for i in 0..256 {
        assert_eq!(dst[i],i as u8);
    }
    
    std::process::exit(0);
}
