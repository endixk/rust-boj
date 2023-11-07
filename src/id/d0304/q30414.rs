// BOJ 30414 [Two-star]
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

fn dfs(adj: &Vec<Vec<usize>>, a: &[i32], b: &[i32], cur: usize, par: usize) -> i32 {
    let mut ret = 0;
    let mut k = 0;

    if a[cur] > b[cur] {
        k = a[cur] - b[cur];
    } else {
        ret += b[cur] - a[cur];
    }

    for &nxt in adj[cur].iter() {
        if nxt == par {
            continue;
        }
        ret += dfs(adj, a, b, nxt, cur);
    }

    if k > ret { ret = 0; } else { ret -= k; }
    ret
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, p) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let a = (0..n).map(|_| next::<i32>(&mut it)).collect::<Vec<_>>();
    let b = (0..n).map(|_| next::<i32>(&mut it)).collect::<Vec<_>>();
    let mut adj = vec![vec![]; n];
    for _ in 1..n {
        let (u, v) = (next::<usize>(&mut it), next::<usize>(&mut it));
        adj[u - 1].push(v - 1);
        adj[v - 1].push(u - 1);
    }

    writeln!(so, "{}", dfs(&adj, &a, &b, p - 1, n))?;

    Ok(())
}
