// BOJ 1629 [Multiplication]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

fn mul(a: i32, p: i32, m: i32) -> i32 {
    let a64 = a as i64;
    let m64 = m as i64;
    if p == 0 {
        1
    } else if p % 2 == 0 {
        let x = mul(a, p / 2, m) as i64;
        (x * x % m64) as i32
    } else {
        let x = mul(a, p - 1, m) as i64;
        (a64 * x % m64) as i32
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let v: Vec<i32> = read(&mut si).split_ascii_whitespace()
        .map(|x| x.parse().unwrap()).collect();
    writeln!(so, "{}", mul(v[0], v[1], v[2])).unwrap();
}
