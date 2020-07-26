//
//  COBS Encode/Decode Function
//
pub fn encode(src: & [u8], slen: usize) -> Vec<u8> {
    let mut i = 0 as usize;
    let mut f = 0 as usize;
    let mut t = 0 as usize;
    let mut v:Vec<u8> = Vec::new();

    while i<slen {
        if src[i]!=0 {
            t+=1;
            if (t-f)>=254 {
                v.push((t-f+1) as u8);
                v.extend_from_slice(&src[f..t]);
                f=t;
            }
        } else {
            v.push((t-f+1) as u8);
            v.extend_from_slice(&src[f..t]);
            f=i+1; t=i+1;
        } 
        i+=1;
    }
    v.push((t-f+1) as u8);
    v.extend_from_slice(&src[f..t]);
    return v;
}

pub fn decode(src:Vec<u8>) -> Vec<u8> {
    let mut v:Vec<u8>=Vec::new();
    let len = src.len();
    let mut i=0 as usize;

    while i<len {
        let l = src[i] as usize;
        if l>1 {
            v.extend_from_slice(&src[i+1..i+l]);
            if l<0xFF {
                v.push(0);
            }
        } else {
            v.push(0);
        }
        i+=l;
    }
    v.pop();
    return v;
}
