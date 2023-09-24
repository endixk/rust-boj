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

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m, k) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let mut d = vec![false; 10];
    let mut dp = vec![0; k];
    for _ in 0..n {
        let i = next::<usize>(&mut it);
        d[i] = true; dp[i%k] += 1;
    }

    const MOD: usize = 1_000_000_007;
    let mut f = 1;
    for x in 1..m {
        f = (f * 10) % k;
        let mut tp = vec![0; k];
        for i in 0..10 {
            if x == m-1 && i == 0 { continue; }
            if d[i] {
                for j in 0..k {
                    tp[(j + i*f) % k] = (tp[(j + i*f) % k] + dp[j]) % MOD;
                }
            }
        }
        dp = tp;
    }
    if m == 1 && d[0] { dp[0] -= 1; }
    writeln!(so, "{}", dp[0])?;

    Ok(())
}