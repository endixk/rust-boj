// BOJ 25195 [Yes or yes]
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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut adj = vec![vec![]; n + 1];
    for _ in 0..next(&mut it) {
        adj[next::<usize>(&mut it)].push(next::<usize>(&mut it));
    }

    let mut fan = vec![false; n + 1];
    for _ in 0..next(&mut it) {
        fan[next::<usize>(&mut it)] = true;
    }
    if fan[1] { writeln!(so, "Yes").ok(); return; }

    let mut vis = vec![false; n + 1];
    let mut q = std::collections::VecDeque::new();
    q.push_back(1);
    while let Some(u) = q.pop_front() {
        let mut flag = false;
        for &v in &adj[u] {
            flag |= fan[v];
            flag |= !vis[v];
            if !fan[v] && !vis[v] {
                vis[v] = true;
                q.push_back(v);
            }
        }
        if !flag { writeln!(so, "yes").ok(); return; }
    }
    writeln!(so, "Yes").ok();
}
