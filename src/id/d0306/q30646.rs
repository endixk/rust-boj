// BOJ 30646 [Pairs with Maximum Sum]
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

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut a = (0..n).map(|i| (next::<i64>(&mut it), i)).collect::<Vec<_>>();
    let mut p = vec![0; n+1];
    for i in 0..n {
        p[i+1] = p[i] + a[i].0;
    }
    a.sort_unstable();

    let (mut ans, mut cnt) = (0, 0);
    let (mut x, mut k) = (a[0].0, a[0].1);
    for i in 1..n {
        if x != a[i].0 {
            let sum = p[a[i-1].1 + 1] - p[k];
            if sum > ans { ans = sum; cnt = 1; }
            else if sum == ans { cnt += 1; }
            x = a[i].0;
            k = a[i].1;
        }
    }
    let sum = p[a[n-1].1 + 1] - p[k];
    if sum > ans { ans = sum; cnt = 1; }
    else if sum == ans { cnt += 1; }

    writeln!(so, "{} {}", ans, cnt)?;

    Ok(())
}
