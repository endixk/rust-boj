// BOJ 1867 [Asteroids]
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

fn bip(adj: &Vec<Vec<usize>>, mat: &mut Vec<usize>, vis: &mut Vec<bool>, cur: usize) -> bool {
    for &nxt in &adj[cur] {
        if vis[nxt] { continue; }
        vis[nxt] = true;
        if mat[nxt] == 0 || bip(adj, mat, vis, mat[nxt]) {
            mat[nxt] = cur;
            return true;
        }
    }
    false
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, k) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut adj = vec![vec![]; n+1];
    for _ in 0..k {
        adj[next::<usize>(&mut it)].push(next::<usize>(&mut it));
    }

    let mut mat = vec![0; n+1];
    let mut ans = 0;
    for i in 1..n+1 {
        let mut vis = vec![false; n+1];
        if bip(&adj, &mut mat, &mut vis, i) {
            ans += 1;
        }
    }
    writeln!(so, "{}", ans)?;

    Ok(())
}
