// BOJ 9095 [Adding 1, 2, 3]
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

    let a = vec![0, 1, 2, 4, 7, 13, 24, 44, 81, 149, 274];
    read(&mut si)
        .split_ascii_whitespace().skip(1)
        .map(|x| a[x.parse::<usize>().unwrap()])
        .for_each(|x| writeln!(so, "{}", x).unwrap());
}
