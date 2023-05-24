// BOJ 1278 [Stageplay]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

// last location of 1 in binary representation of n
fn lo(n: usize) -> u8 {
    let mut i = 0;
    while n & (1<<i) == 0 {
        i += 1;
    }
    i
}

pub fn main() {
    let mut so = io::BufWriter::new(io::stdout().lock());
    let n = io::stdin().lock().lines().next().unwrap().unwrap().parse::<usize>().unwrap();

    if n == 1 {
        writeln!(so, "1 1 1").ok();
    } else if n == 2 {
        writeln!(so, "3 1 2 1 2").ok();
    } else {
        write!(so, "{} ", (1<<n)-1).ok();
        for n in 1..=(1<<n-3) {
            write!(so, "1 2 1 {} ", lo(n)+3).ok();
        }
        for n in 1..=(1<<n-3) {
            write!(so, "1 2 1 {} ", lo(n)+3).ok();
        }
        writeln!(so, "").ok();
    }
}
