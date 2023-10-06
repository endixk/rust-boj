// BOJ 1450 [Knapsack Problem]
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

#[inline] fn get(a: &[i64], n: usize, bit: u32) -> i64 {
    let mut ret = 0;
    for i in 0..n {
        if bit & (1 << i) != 0 {
            ret += a[i];
        }
    }
    ret
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, c) = (next::<usize>(&mut it), next::<i64>(&mut it));
    let a = (0..n/2).map(|_| next::<i64>(&mut it)).collect::<Vec<_>>();
    let b = (n/2..n).map(|_| next::<i64>(&mut it)).collect::<Vec<_>>();

    let mut w = Vec::new();
    for bit in 0..(1 << n/2) {
        w.push(get(&a, n/2, bit));
    }
    w.sort_unstable();

    let mut ans = 0;
    for bit in 0..(1 << (n - n/2)) {
        let v = get(&b, n - n/2, bit);
        if v > c { continue; }
        ans += w.partition_point(|&x| x <= c - v);
    }
    writeln!(so, "{}", ans)?;

    Ok(())
}
