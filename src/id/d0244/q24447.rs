// BOJ 24447 [Breadth-First Search 4]
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

    let (n, m, r) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let mut adj = vec![vec![]; n+1];
    for _ in 0..m {
        let (u, v) = (next::<usize>(&mut it), next::<usize>(&mut it));
        adj[u].push(v);
        adj[v].push(u);
    }
    for v in adj.iter_mut() { v.sort_unstable(); }

    let mut q = std::collections::VecDeque::new();
    let mut d = vec![-1; n+1];
    let mut i = 1; let mut j = 1; d[r] = 0; q.push_back(r);
    while !q.is_empty() {
        let sz = q.len();
        for _ in 0..sz {
            let u = q.pop_front().unwrap();
            for &v in &adj[u] {
                if d[v] >= 0 { continue; }
                j += 1; d[v] = i * j; q.push_back(v);
            }
        }
        i += 1;
    }
    writeln!(so, "{}", d.iter().filter(|&&x| x > 0).sum::<i64>()).ok();

    Ok(())
}
