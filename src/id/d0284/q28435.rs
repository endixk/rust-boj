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

const MOD: usize = 1_000_000_007;
fn pow(a: usize, b: usize) -> usize {
    if b == 0 { return 1; }
    let mut r = pow(a, b / 2);
    r = (r * r) % MOD;
    if b % 2 == 1 { r = (r * a) % MOD; }
    r
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, k) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut m = vec![0; k];
    for _ in 0..n {
        m[next::<usize>(&mut it) % k] += 1;
    }

    let mut ans = 1;
    ans = (ans * (m[0] + 1)) % MOD;
    if k % 2 == 0 {
        ans = (ans * (m[k / 2] + 1)) % MOD;
    }
    for i in 1..(k+1)/2 {
        let r = (pow(2, m[i]) + pow(2, m[k - i]) - 1) % MOD;
        ans = (ans * r) % MOD;
    }
    ans = (ans + MOD - n - 1) % MOD;
    writeln!(so, "{}", ans).unwrap();
}
