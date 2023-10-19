// BOJ 30206 [Vehicle Arrangement]
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
    let mut adj = vec![vec![]; n+1];
    for _ in 0..next(&mut it) {
        let (u, v) = (next::<usize>(&mut it), next::<usize>(&mut it));
        adj[u].push(v);
        adj[v].push(u);
    }

    const MOD: i64 = 1_000_000_007;
    let mut ans = 1;
    let mut q = std::collections::VecDeque::new();
    let mut vis = vec![false; n+1];
    q.push_back(1); vis[1] = true;
    while !q.is_empty() {
        let sz = q.len() as i64;
        ans = (ans * sz + ans) % MOD;
        for _ in 0..sz {
            let u = q.pop_front().unwrap();
            for &v in &adj[u] {
                if !vis[v] {
                    q.push_back(v);
                    vis[v] = true;
                }
            }
        }
    }
    writeln!(so, "{}", ans - 1)?;

    Ok(())
}
