// BOJ 13141 [Ignition]
// Supported by GitHub Copilot

use std::io::{self, Read};

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

const INF: i32 = 0x3f3f3f3f;
fn floyd(adj: &mut Vec<Vec<i32>>, n: usize) {
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                adj[i][j] = adj[i][j].min(adj[i][k] + adj[k][j]);
            }
        }
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut adj = vec![vec![INF; n]; n];
    (0..n).for_each(|i| adj[i][i] = 0);

    let mut mdj = vec![vec![0; n]; n];
    for _ in 0..m {
        let (u, v, w) = (
            next::<usize>(&mut it) - 1,
            next::<usize>(&mut it) - 1,
            next::<i32>(&mut it));
        adj[u][v] = adj[u][v].min(w);
        adj[v][u] = adj[v][u].min(w);
        mdj[u][v] = mdj[u][v].max(w);
        mdj[v][u] = mdj[v][u].max(w);
    }

    floyd(&mut adj, n);
    let mut ans = INF;
    for i in 0..n {
        let mut t = 0;
        for j in 0..n {
            t = t.max(adj[i][j] << 1);
            for k in j..n {
                if mdj[j][k] == 0 { continue; }
                let (p, q) = if adj[i][j] > adj[i][k] { (adj[i][j], adj[i][k]) } else { (adj[i][k], adj[i][j]) };
                if p - q > mdj[j][k] {
                    t = t.max((adj[i][j] + mdj[j][k]) << 1);
                } else {
                    t = t.max((p << 1) + mdj[j][k] - p + q);
                }
            }
        }
        ans = ans.min(t);
    }

    println!("{}{}", ans >> 1, if ans & 1 == 1 { ".5" } else { ".0" });
}
