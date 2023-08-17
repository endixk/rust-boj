// BOJ 13896 [Sky Tax]
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

fn tour(et: &mut Vec<usize>, id: &mut Vec<usize>, sp: &mut Vec<Vec<usize>>, x: &mut usize,
        adj: &Vec<Vec<usize>>, u: usize, p: usize) {
    *x += 1;
    id[u] = *x;
    sp[id[u]][0] = id[p];
    for i in 1..20 {
        sp[id[u]][i] = sp[sp[id[u]][i-1]][i-1];
    }
    for &v in &adj[u] {
        if id[v] == 0 {
            tour(et, id, sp, x, adj, v, u);
        }
    }
    et[id[u]] = *x - id[u];
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    for tc in 1..=next(&mut it) {
        let (n, q, mut r) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
        let mut adj = vec![vec![]; n+1];
        for _ in 1..n {
            let (u, v) = (next::<usize>(&mut it), next::<usize>(&mut it));
            adj[u].push(v);
            adj[v].push(u);
        }

        let mut et = vec![0; n+1];
        let mut id = vec![0; n+1];
        let mut sp = vec![vec![0; 20]; n+1];
        let mut x = 0;
        tour(&mut et, &mut id, &mut sp, &mut x, &adj, 1, 0);

        writeln!(so, "Case #{}:", tc).unwrap();
        for _ in 0..q {
            let (s, u) = (next::<usize>(&mut it), next::<usize>(&mut it));
            if s == 0 { r = u; continue; }
            if r == u { writeln!(so, "{}", n).ok(); continue; }

            let (r, u) = (id[r], id[u]);
            if r > u && r <= u + et[u] {
                let mut p = r;
                while sp[p][0] > u {
                    let i = (0..20).rev().find(|&i| sp[p][i] > u).unwrap();
                    p = sp[p][i];
                }
                writeln!(so, "{}", n - et[p] - 1).ok();
            } else {
                writeln!(so, "{}", et[u] + 1).ok();
            }
        }
    }
}
