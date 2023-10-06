// BOJ 13913 [Hide and Seek 4]
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

    let (s, e) = (next::<usize>(&mut it), next::<usize>(&mut it));
    const SZ: usize = 199999;

    let mut par = vec![0; SZ];
    let mut vis = vec![false; SZ];
    let mut q = std::collections::VecDeque::new();
    q.push_back(s); vis[s] = true; par[s] = s;
    while let Some(u) = q.pop_front() {
        if u == e { break; }
        if u > 0 && !vis[u-1] {
            q.push_back(u-1); vis[u-1] = true; par[u-1] = u;
        }
        if u < SZ-1 && !vis[u+1] {
            q.push_back(u+1); vis[u+1] = true; par[u+1] = u;
        }
        if u*2 < SZ && !vis[u*2] {
            q.push_back(u*2); vis[u*2] = true; par[u*2] = u;
        }
    }

    q.clear();
    let mut u = e;
    while u != s {
        q.push_back(u);
        u = par[u];
    }

    writeln!(so, "{}", q.len())?;
    write!(so, "{} ", s)?;
    while let Some(u) = q.pop_back() {
        write!(so, "{} ", u)?;
    }
    writeln!(so)?;

    Ok(())
}
