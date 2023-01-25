// BOJ 11399 [ATM]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let mut v: Vec<i32> = read(&mut si)
        .split_ascii_whitespace().skip(1)
        .map(|x| x.parse().unwrap()).collect();
    v.sort();
    writeln!(so, "{}", v.iter().rev()
        .zip((1..).map(|x| x as i32))
        .fold(0, |a, (x, y)| a + x * y)).unwrap();
}
