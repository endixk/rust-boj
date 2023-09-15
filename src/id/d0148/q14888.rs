// BOJ 14888 [Insert Operators]
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

static mut ANS_MIN: i64 = i64::MAX;
static mut ANS_MAX: i64 = i64::MIN;
fn go(a: &[i64], c: &[u8], x: i64, i: usize, n: usize, plu: u8, sub: u8, mul: u8, div: u8) {
    if i == n {
        unsafe {
            ANS_MIN = ANS_MIN.min(x);
            ANS_MAX = ANS_MAX.max(x);
        }
        return;
    }
    if plu < c[0] { go(a, c, x + a[i], i + 1, n, plu + 1, sub, mul, div); }
    if sub < c[1] { go(a, c, x - a[i], i + 1, n, plu, sub + 1, mul, div); }
    if mul < c[2] { go(a, c, x * a[i], i + 1, n, plu, sub, mul + 1, div); }
    if div < c[3] { go(a, c, x / a[i], i + 1, n, plu, sub, mul, div + 1); }
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let a = (0..n).map(|_| next::<i64>(&mut it)).collect::<Vec<_>>();
    let c = (0..4).map(|_| next::<u8>(&mut it)).collect::<Vec<_>>();
    go(&a, &c, a[0], 1, n, 0, 0, 0, 0);

    unsafe {
        writeln!(so, "{}\n{}", ANS_MAX, ANS_MIN).unwrap();
    }
}
