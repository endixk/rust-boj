// BOJ 30509 [Coding Gauntlet]
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

fn go(l: i64, r: i64, s: i64, m: i64, u: &[i64], v: &[i64], k: usize, n: usize) -> bool {
    let mut m = m + r * k as i64;
    let mut i = 0;
    let mut pq = std::collections::BinaryHeap::new();
    while i < n && u[i] + v[i] <= s + m {
        pq.push((u[i], v[i]));
        i += 1;
    }

    let mut c = 0;
    while !pq.is_empty() {
        let (x, y) = pq.pop().unwrap();
        if x + y < s + m && x > s { m += x - s; }
        c += 1;
        if c == n - k { return true; }
        while i < n && u[i] + v[i] <= s + m {
            pq.push((u[i], v[i]));
            i += 1;
        }
    }

    let mut w = u[i..].to_vec();
    w.sort_unstable_by(|a, b| b.cmp(a));
    i = 0;
    while c < n - k {
        if w[i] >= s { m -= l; }
        else { m -= l + s - w[i]; }
        if m <= 0 { return false; }
        i += 1; c += 1;
    }

    true
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, l, r) = (next::<usize>(&mut it), next::<i64>(&mut it), next::<i64>(&mut it));
    let (s, m) = (next::<i64>(&mut it), next::<i64>(&mut it));

    let mut v = (0..n).map(|_| (next::<i64>(&mut it), next::<i64>(&mut it))).collect::<Vec<_>>();
    v.sort_unstable_by(|a, b| (a.0 + a.1).cmp(&(b.0 + b.1)));
    let u = v.iter().map(|x| x.0).collect::<Vec<_>>();
    let v = v.iter().map(|x| x.1).collect::<Vec<_>>();

    let (mut lo, mut hi) = (0, n);
    while lo < hi {
        let mid = (lo + hi) / 2;
        if !go(l, r, s, m, &u, &v, mid, n) { lo = mid + 1; }
        else { hi = mid; }
    }
    writeln!(so, "{}", n-lo)?;

    Ok(())
}
