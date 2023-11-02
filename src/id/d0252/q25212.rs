// BOJ 25212 [Pieces of Cake]
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

fn go(c: &[i32], bit: u16, n: usize) -> bool {
    let mut f = 0.0;
    for i in 0..n {
        if bit & (1 << i) != 0 {
            f += 1.0 / c[i] as f64;
        }
    }
    0.99 <= f && f <= 1.01
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let c = (0..n).map(|_| next::<i32>(&mut it)).collect::<Vec<_>>();
    writeln!(so, "{}", (1..1<<n).map(|bit| go(&c, bit, n) as i32).sum::<i32>()).ok();

    Ok(())
}
