// BOJ 30511 [Skewers]
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

#[inline] fn get(b: &Vec<Vec<char>>, x: i32, y: i32, dx: i32, dy: i32, n: usize, m: usize) -> char {
    if x + dx < 0 || x + dx >= n as i32 || y + dy < 0 || y + dy >= m as i32 { return 'X'; }
    b[(x + dx) as usize][(y + dy) as usize]
}
const DX: [i32; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];
const DY: [i32; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut b = vec![vec![' '; m]; n];
    for i in 0..n {
        let s = next::<String>(&mut it);
        for (j, c) in s.chars().enumerate() {
            b[i][j] = c;
        }
    }

    use std::collections::HashMap;
    let (mut gmap, mut smap) = (HashMap::new(), HashMap::new());
    let (mut gi, mut si) = (1, 1);
    for i in 0..n { for j in 0..m {
        if b[i][j] == 'G' { gmap.insert(i*m+j, gi); gi += 1; }
        if b[i][j] == 'S' { smap.insert(i*m+j, si); si += 1; }
    }}

    let mut adj = vec![vec![]; gi];
    for i in 0..n { for j in 0..m {
        if b[i][j] != 'G' { continue; }
        let (x, y) = (i as i32, j as i32);
        for (&dx, &dy) in DX.iter().zip(DY.iter()) {
            if get(&b, x, y, dx, dy, n, m) == 'S' {
                if get(&b, x, y, dx*2, dy*2, n, m) == 'M' || get(&b, x, y, -dx, -dy, n, m) == 'M' {
                    adj[gmap[&(i*m+j)]].push(smap[&((x+dx) as usize*m+(y+dy) as usize)]);
                }
            }
        }
    }}

    let mut mat = vec![0; si];
    let mut ans = 0;
    for i in 1..gi {
        let mut vis = vec![false; si];
        if bip(&adj, &mut mat, &mut vis, i) { ans += 1; }
    }
    writeln!(so, "{}", ans)?;

    Ok(())
}
