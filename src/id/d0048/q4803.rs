// BOJ 4803 [Trees]
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

fn dfs(adj: &Vec<Vec<usize>>, vis: &mut Vec<bool>, p: usize, u: usize) -> bool {
    let mut ret = true;
    vis[u] = true;
    for &v in &adj[u] {
        if vis[v] && v != p { return false; }
        if vis[v] { continue; }
        ret &= dfs(adj, vis, u, v);
    }
    ret
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let mut tc = 1;
    loop {
        let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
        if n == 0 && m == 0 { break; }

        let mut adj = vec![vec![]; n + 1];
        for _ in 0..m {
            let (u, v) = (next::<usize>(&mut it), next::<usize>(&mut it));
            adj[u].push(v);
            adj[v].push(u);
        }
        let mut vis = vec![false; n + 1];
        let mut cnt = 0;
        for i in 1..=n {
            if vis[i] { continue; }
            if dfs(&adj, &mut vis, 0, i) { cnt += 1; }
        }

        match cnt {
            0 => writeln!(so, "Case {}: No trees.", tc)?,
            1 => writeln!(so, "Case {}: There is one tree.", tc)?,
            _ => writeln!(so, "Case {}: A forest of {} trees.", tc, cnt)?,
        }
        tc += 1;
    }

    Ok(())
}
