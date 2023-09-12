// BOJ 2570 [Bishop 2]
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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut b = vec![vec![false; n]; n];
    for _ in 0..m {
        b[next::<usize>(&mut it) - 1][next::<usize>(&mut it) - 1] = true;
    }

    let mut li = vec![vec![0; n]; n];
    let mut id = 0;
    for j in 0..n {
        id += 1;
        for i in 0..=j {
            if b[i][j-i] { id += 1; continue; }
            li[i][j-i] = id;
        }
    }
    for i in 1..n {
        id += 1;
        for j in 0..n-i {
            if b[i+j][n-1-j] { id += 1; continue; }
            li[i+j][n-1-j] = id;
        }
    }

    let k = id + 1;
    let mut adj = vec![vec![]; k];
    for j in 0..n {
        id += 1;
        for i in 0..n-j {
            if b[i][j+i] { id += 1; continue; }
            adj[li[i][j+i]].push(id);
        }
    }
    for i in 1..n {
        id += 1;
        for j in 0..n-i {
            if b[i+j][j] { id += 1; continue; }
            adj[li[i+j][j]].push(id);
        }
    }

    let mut mat = vec![0; id+1];
    for i in 1..k {
        let mut vis = vec![false; id+1];
        bip(&adj, &mut mat, &mut vis, i);
    }
    writeln!(so, "{}", mat.iter().filter(|&&x| x != 0).count()).unwrap();
}
