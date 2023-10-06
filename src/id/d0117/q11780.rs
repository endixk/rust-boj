// BOJ 11780 [Floyd-Warshall 2]
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
    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut adj = vec![vec![INF; n]; n];
    for _ in 0..m {
        let (u, v, w) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<i32>(&mut it));
        adj[u-1][v-1] = adj[u-1][v-1].min(w);
    }
    for i in 0..n { adj[i][i] = 0; }

    let mut trk = vec![vec![0; n]; n];
    for i in 0..n { for j in 0..n {
        trk[i][j] = i;
    }}

    for k in 0..n { for i in 0..n { for j in 0..n {
        if adj[i][j] > adj[i][k] + adj[k][j] {
            adj[i][j] = adj[i][k] + adj[k][j];
            trk[i][j] = trk[k][j];
        }
    }}}

    for i in 0..n {
        for j in 0..n {
            write!(so, "{} ", if adj[i][j] == INF { 0 } else { adj[i][j] })?;
        }
        writeln!(so)?;
    }

    for i in 0..n {
        for j in 0..n {
            if i == j { writeln!(so, "0")?; continue; }
            if adj[i][j] == INF { writeln!(so, "0")?; continue; }
            let mut v = Vec::new();
            let mut u = j;
            while u != i {
                v.push(u);
                u = trk[i][u];
            }
            v.push(i);
            write!(so, "{} ", v.len())?;
            while let Some(u) = v.pop() {
                write!(so, "{} ", u+1)?;
            }
            writeln!(so)?;
        }
    }

    Ok(())
}
