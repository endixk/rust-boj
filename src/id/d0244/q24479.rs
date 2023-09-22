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

fn dfs(x: &mut [i64], d: &mut [i64], adj: &[Vec<usize>], u: usize, k: i64, i: &mut i64) {
    x[u] = *i; *i += 1; d[u] = k;
    for &v in adj[u].iter() {
        if d[v] == -1 { dfs(x, d, adj, v, k+1, i); }
    }
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

    let mut x = vec![0; n+1];
    let mut d = vec![-1; n+1];
    let mut i = 1;
    dfs(&mut x, &mut d, &adj, r, 0, &mut i);

    writeln!(so, "{}", x.iter().zip(d.iter()).skip(1).map(|(&x, &y)| x*y).sum::<i64>())?;

    Ok(())
}
