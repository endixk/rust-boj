// BOJ 28245 [I'm Hungry (Hard)]
// Supported by GitHub Copilot

use std::io::{self, Read, Write};

fn read<T>(si: &mut T) -> String where T: Read {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

fn next<T>(it: &mut std::str::SplitAsciiWhitespace) -> T where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {
    it.next().unwrap().parse().unwrap()
}

fn cbit(x: u64) -> Vec<bool> {
    let mut v = Vec::new();
    let mut x = x;
    while x > 0 {
        v.push(x & 1 == 1);
        x >>= 1;
    }
    v
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    for _ in 0..next(&mut it) {
        let x = next::<u64>(&mut it);
        if x == 1 { writeln!(so, "0 0").ok(); continue; }

        let bits = cbit(x);
        let c = bits.iter().filter(|&&x| x).count();
        if c == 1 {
            writeln!(so, "{} {}", bits.len()-2, bits.len()-2).ok();
            continue;
        }

        let p = bits.len()-1;
        let mut q = bits.len()-2;
        while q > 0 && !bits[q] { q -= 1; }
        if c == 2 {
            writeln!(so, "{} {}", q, p).ok();
            continue;
        }

        let l = (1 << p) + (1 << q);
        let r = (1 << p) + (1 << (q+1));
        if x - l <= r - x {
            writeln!(so, "{} {}", q, p).ok();
        } else {
            writeln!(so, "{} {}", q+1, p).ok();
        }
    }
}

