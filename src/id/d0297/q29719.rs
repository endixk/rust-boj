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

const MOD: i64 = 1_000_000_007;
fn pow(a: i64, b: i64) -> i64 {
    let mut r = 1;
    let mut a = a;
    let mut b = b;
    while b > 0 {
        if b & 1 == 1 {
            r = r * a % MOD;
        }
        a = a * a % MOD;
        b >>= 1;
    }
    r
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<i64>(&mut it), next::<i64>(&mut it));
    writeln!(so, "{}", (pow(m, n) + MOD - pow(m-1, n)) % MOD).ok();
}