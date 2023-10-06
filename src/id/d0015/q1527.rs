// BOJ 1527 [Golden Numbers]
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

fn get(bit: u32, n: usize) -> u32 {
    let mut ret = 0;
    for i in 0..n {
        if bit & (1 << i) != 0 {
            ret = ret * 10 + 7;
        } else {
            ret = ret * 10 + 4;
        }
    }
    ret
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let mut k = Vec::new();
    for n in 1..10 {
        for bit in 0..(1 << n) {
            k.push(get(bit, n));
        }
    }
    k.sort_unstable();

    let (a, b) = (next::<u32>(&mut it), next::<u32>(&mut it));
    let a = k.binary_search(&a).unwrap_or_else(|x| x);
    let b = match k.binary_search(&b) {
        Ok(x) => x + 1,
        Err(x) => x,
    };
    writeln!(so, "{}", b - a)?;

    Ok(())
}
