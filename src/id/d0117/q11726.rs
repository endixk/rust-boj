// BOJ 11726 [2 by N Tiling]
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

    let mut fib = vec![1, 1];
    for i in 1..read(&mut si).trim().parse::<usize>().unwrap() {
        fib.push((fib[i-1] + fib[i]) % 10_007);
    }
    writeln!(so, "{}", fib.last().unwrap()).unwrap();
}
