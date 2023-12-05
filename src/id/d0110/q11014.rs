// BOJ 11014 [No Cheating]
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

const DX: [i32; 6] = [-1, 0, 1, -1, 0, 1];
const DY: [i32; 6] = [-1, -1, -1, 1, 1, 1];
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    for _ in 0..next::<usize>(&mut it) {
        let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
        let mut b = vec![vec![false; m]; n];
        let mut a = 0;
        for i in 0..n {
            let s = next::<String>(&mut it);
            for (j, c) in s.chars().enumerate() {
                if c == '.' { b[i][j] = true; a += 1; }
            }
        }

        let mut x = vec![vec![0; m]; n];
        let (mut p, mut q) = (1, 1);
        for i in 0..n {
            for j in 0..m {
                if j % 2 == 0 { x[i][j] = p; p += 1; }
                else { x[i][j] = q; q += 1; }
            }
        }

        let mut adj = vec![vec![]; p];
        for i in 0..n {
            for j in 0..m {
                if j % 2 != 0 { continue; }
                if !b[i][j] { continue; }
                for (&dx, &dy) in DX.iter().zip(DY.iter()) {
                    let (nx, ny) = (i as i32 + dx, j as i32 + dy);
                    if nx < 0 || nx >= n as i32 || ny < 0 || ny >= m as i32 { continue; }
                    if !b[nx as usize][ny as usize] { continue; }
                    adj[x[i][j]].push(x[nx as usize][ny as usize]);
                }
            }
        }

        let mut mat = vec![0; q];
        let mut ans = 0;
        for i in 1..p {
            let mut vis = vec![false; q];
            if bip(&adj, &mut mat, &mut vis, i) {
                ans += 1;
            }
        }
        writeln!(so, "{}", a - ans)?;
    }

    Ok(())
}
