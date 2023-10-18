// BOJ 3665 [Ranking]
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

    'tc: for _ in 0..next(&mut it) {
        let n = next::<usize>(&mut it);
        let r = (0..n).map(|_| next::<usize>(&mut it)-1).collect::<Vec<_>>();

        let mut adj = vec![vec![false; n]; n];
        for i in 0..n-1 {
            for j in i+1..n {
                adj[r[i]][r[j]] = true;
            }
        }

        for _ in 0..next(&mut it) {
            let (a, b) = (next::<usize>(&mut it)-1, next::<usize>(&mut it)-1);
            adj[a][b] = !adj[a][b];
            adj[b][a] = !adj[b][a];
        }

        let mut ind = vec![0; n];
        let mut q = std::collections::VecDeque::new();
        for i in 0..n {
            ind[i] = (0..n).filter(|&j| adj[j][i]).count();
            if ind[i] == 0 { q.push_back(i); }
        }

        let mut ans = Vec::new();
        while let Some(u) = q.pop_front() {
            if !q.is_empty() {
                writeln!(so, "?").ok();
                continue 'tc;
            }

            ans.push(u);
            for v in 0..n {
                if adj[u][v] {
                    ind[v] -= 1;
                    if ind[v] == 0 { q.push_back(v); }
                }
            }
        }

        if ans.len() != n {
            writeln!(so, "IMPOSSIBLE").ok();
        } else {
            ans.iter().for_each(|&x| write!(so, "{} ", x+1).unwrap());
            writeln!(so).ok();
        }
    }

    Ok(())
}
