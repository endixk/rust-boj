// BOJ 28465 [Fibonacci Squares]
// Supported by GitHub Copilot

use std::io::{self, Read};
fn read_int(si: &mut dyn Read) -> i64 {
    let mut buf = [0u8; 10];
    let mut i = 0;
    loop {
        si.read_exact(&mut buf[i..i+1]).unwrap();
        if buf[i] == b' ' || buf[i] == b'\n' { break; }
        i += 1;
    }
    std::str::from_utf8(&buf[..i]).unwrap().parse().unwrap()
}
fn go(a: i64, b: i64, x: i64, y: i64, n: usize) -> usize {
    if n < 3 { return 1; }
    if a < b {
        return go(b, a, b-y-1, x, n);
    }
    return if x < a-b { go(b, a-b, b-y-1, x, n-1) }
    else if x < b { n }
    else { go(b, a-b, b-y-1, x-b, n-1) }
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let (a, b) = (read_int(&mut si), read_int(&mut si));

    let (mut p, mut q, mut n) = (1, 1, 1);
    while p < a && p < b {
        (p, q) = (p+q, p);
        n += 1;
    }
    println!("{}", go(a, b, read_int(&mut si), read_int(&mut si), n+1));
}
