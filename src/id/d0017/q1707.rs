// BOJ 1707 [Bipartite Graph]
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
        let (v, e) = (next::<usize>(&mut it), next::<usize>(&mut it));
        let mut adj = vec![vec![]; v];
        for _ in 0..e {
            let (i, j) = (next::<usize>(&mut it) - 1, next::<usize>(&mut it) - 1);
            adj[i].push(j);
            adj[j].push(i);
        }

        let mut p = vec![0i8; v];
        let mut q = std::collections::VecDeque::new();
        for i in 0..v {
            if p[i] != 0 { continue; }
            q.push_back(i); p[i] = 1;
            while let Some(i) = q.pop_front() {
                for &j in &adj[i] {
                    if p[j] == 0 {
                        p[j] = -p[i];
                        q.push_back(j);
                    }
                }
            }
        }

        for i in 0..v {
            for &j in &adj[i] {
                if p[i] == p[j] {
                    writeln!(so, "NO")?;
                    continue 'tc;
                }
            }
        }
        writeln!(so, "YES")?;
    }

    Ok(())
}
