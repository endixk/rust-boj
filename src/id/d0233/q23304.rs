use std::io::{self, Read};
fn read<T>(si: &mut T) -> String where T: Read {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s.pop();
    s
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut l = s.len();
    while l > 1 {
        if s[..l>>1] != s[l+1>>1..(l+1>>1)+(l>>1)] { println!("IPSELENTI"); return; }
        l >>= 1;
    }
    println!("AKARAKA");
}
