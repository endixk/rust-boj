// BOJ 17472 [Building Bridges]
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

const DX: [i32; 4] = [0, 0, -1, 1];
const DY: [i32; 4] = [-1, 1, 0, 0];
fn fill(map: &mut Vec<Vec<i8>>, i: usize, j: usize, c: i8) {
    let mut q = std::collections::VecDeque::new();
    q.push_back((i, j));
    map[i][j] = c;
    while let Some((x, y)) = q.pop_front() {
        for (&dx, &dy) in DX.iter().zip(DY.iter()) {
            let (x, y) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
            if map[x][y] == -1 {
                map[x][y] = c;
                q.push_back((x, y));
            }
        }
    }
}
fn search(map: &Vec<Vec<i8>>, i: usize, j: usize, d: i8) -> Option<(i8, i32)> {
    let (mut i, mut j, mut k) = (i, j, 0);
    match d { 0 => i -= 1, 1 => i += 1, 2 => j -= 1, _ => j += 1 }
    while map[i][j] == 0 {
        k += 1;
        match d { 0 => i -= 1, 1 => i += 1, 2 => j -= 1, _ => j += 1 }
    }

    return if k > 1 && map[i][j] > 0 {
        Some((map[i][j], k))
    } else { None }
}
fn find(root: &mut Vec<usize>, x: usize) -> usize {
    if root[x] == x {
        x
    } else {
        root[x] = find(root, root[x]);
        root[x]
    }
}
fn union(root: &mut Vec<usize>, rank: &mut Vec<usize>, x: usize, y: usize) {
    let x = find(root, x);
    let y = find(root, y);
    if x == y {
        return;
    }
    if rank[x] < rank[y] {
        root[x] = y;
    } else {
        root[y] = x;
        if rank[x] == rank[y] {
            rank[x] += 1;
        }
    }
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut map = vec![vec![-2; m+2]; n+2];
    for i in 1..=n { for j in 1..=m {
        map[i][j] = -next::<i8>(&mut it);
    }}

    let mut c = 1;
    for i in 1..=n { for j in 1..=m {
        if map[i][j] == -1 {
            fill(&mut map, i, j, c);
            c += 1;
        }
    }}

    let mut edges = Vec::new();
    for i in 1..=n { for j in 1..=m {
        if map[i][j] == 0 { continue; }
        for d in 0..4 {
            if let Some((c, k)) = search(&map, i, j, d) {
                edges.push((k, map[i][j], c));
            }
        }
    }}
    edges.sort_unstable();

    let mut ans = 0;
    let mut root = (0..c as usize).collect::<Vec<_>>();
    let mut rank = vec![0; c as usize];
    for (w, u, v) in edges {
        let (u, v) = (find(&mut root, u as usize), find(&mut root, v as usize));
        if u != v {
            union(&mut root, &mut rank, u, v);
            ans += w;
        }
    }
    for i in 2..c {
        if find(&mut root, 1) != find(&mut root, i as usize) {
            ans = -1;
            break;
        }
    }
    writeln!(so, "{}", ans)?;

    Ok(())
}
