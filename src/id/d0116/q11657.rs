// BOJ 11657 [Time Machine]
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

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let e = (0..m).map(|_|
        (next::<usize>(&mut it), next::<usize>(&mut it), next::<i64>(&mut it))
    ).collect::<Vec<_>>();
    const INF: i64 = 0x3f3f3f3f3f3f3f3f;

    let mut d = vec![INF; n+1];
    d[1] = 0;
    for _ in 1..n {
        for &(u, v, w) in &e {
            if d[u] != INF && d[v] > d[u] + w {
                d[v] = d[u] + w;
            }
        }
    }
    for &(u, v, w) in &e {
        if d[u] != INF && d[v] > d[u] + w {
            writeln!(so, "-1")?;
            return Ok(());
        }
    }

    for i in 2..=n {
        if d[i] == INF { writeln!(so, "-1")?; }
        else { writeln!(so, "{}", d[i])?; }
    }

    Ok(())
}
