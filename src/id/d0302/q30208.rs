// BOJ 30208 [Leave]
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

    let (n, s) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut w = vec![0; n+1];
    for i in 1..=n { w[i] = next::<usize>(&mut it); }
    let mut t = vec![0; n+1];
    for i in 1..=n { t[i] = next::<usize>(&mut it); }

    let (mut nxt, mut prv) = (vec![0; n+1], vec![0; n+1]);
    for i in 1..=n {
        let p = next::<usize>(&mut it);
        nxt[p] = i; prv[i] = p;
    }

    let mut grp = Vec::new();
    for i in 1..=n {
        if prv[i] != 0 { continue; }
        let (mut gw, mut gt) = (0, 0);
        let mut v = Vec::new();
        let mut j = i;
        while j != 0 {
            gw += w[j]; gt += t[j];
            v.push((gw, gt));
            j = nxt[j];
        }
        grp.push(v);
    }

    const INF: usize = 0x3f3f3f3f;
    const MAX: usize = 100001;
    let mut dp = vec![INF; MAX];
    dp[0] = 0;
    for g in grp {
        let mut tp = dp.clone();
        for (w, t) in g {
            for i in 0..MAX {
                if i + w >= MAX { break; }
                if dp[i] == INF { continue; }
                tp[i+w] = tp[i+w].min(dp[i] + t);
            }
        }
        dp = tp;
    }

    let ans = *dp.iter().skip(s).min().unwrap();
    writeln!(so, "{}", if ans == INF { -1 } else { ans as i32 })?;

    Ok(())
}
