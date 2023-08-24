// BOJ 28464 [Potato]
// Supported by GitHub Copilot

use std::io::Read;
fn read_int(si: &mut dyn Read) -> usize {
    let mut buf = [0u8; 10];
    let mut i = 0;
    loop {
        si.read_exact(&mut buf[i..i+1]).unwrap();
        if buf[i] == b' ' || buf[i] == b'\n' { break; }
        i += 1;
    }
    std::str::from_utf8(&buf[..i]).unwrap().parse().unwrap()
}
pub fn main() {
    let mut si = std::io::BufReader::with_capacity(1<<16, std::io::stdin().lock());
    let n = read_int(&mut si);
    let mut c = vec![0; 10001];
    let mut sum = 0;
    (0..n).for_each(|_| { let x = read_int(&mut si); c[x] += 1; sum += x; });
    let (mut i, mut k, mut s) = (0, 0, 0);
    while k + c[i] < n >> 1 { k += c[i]; s += c[i] * i; i += 1; }
    s += ((n >> 1) - k) * i;
    println!("{} {}", s, sum - s);
}
