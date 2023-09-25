// BOJ 1956 [Exercise]
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

    const INF: i32 = 0x3f3f3f3f;
    let (v, e) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut adj = vec![vec![INF; v]; v];
    for _ in 0..e {
        let (a, b, c) = (next::<usize>(&mut it)-1, next::<usize>(&mut it)-1, next::<i32>(&mut it));
        adj[a][b] = c;
    }
    for i in 0..v { adj[i][i] = 0; }
    for k in 0..v {
        for i in 0..v {
            for j in 0..v {
                adj[i][j] = adj[i][j].min(adj[i][k] + adj[k][j]);
            }
        }
    }

    let mut ans = INF;
    for i in 0..v-1 {
        for j in i+1..v {
            if adj[i][j] != INF && adj[j][i] != INF {
                ans = ans.min(adj[i][j] + adj[j][i]);
            }
        }
    }
    if ans == INF { writeln!(so, "-1")?; }
    else { writeln!(so, "{}", ans)?; }

    Ok(())
}
