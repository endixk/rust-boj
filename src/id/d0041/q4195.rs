// BOJ 4195 [Virtual Friends]
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

fn find(root: &mut Vec<usize>, x: usize) -> usize {
    if root[x] == x {
        x
    } else {
        root[x] = find(root, root[x]);
        root[x]
    }
}
fn union_size(root: &mut Vec<usize>,
              rank: &mut Vec<usize>,
              size: &mut Vec<usize>, x: usize, y: usize) -> usize {
    let x = find(root, x);
    let y = find(root, y);
    if x != y {
        if rank[x] < rank[y] {
            root[x] = y;
            size[y] += size[x];
            size[y]
        } else {
            root[y] = x;
            size[x] += size[y];
            if rank[x] == rank[y] {
                rank[x] += 1;
            }
            size[x]
        }
    } else {
        size[x]
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    for _ in 0..next(&mut it) {
        let f = next::<usize>(&mut it);
        let mut map = std::collections::HashMap::new();
        let mut c = 0;
        let mut root = (0..(f<<1)).collect::<Vec<usize>>();
        let mut rank = vec![0; f<<1];
        let mut size = vec![1; f<<1];
        for _ in 0..f {
            let a = *map.entry(next::<String>(&mut it)).or_insert({ c += 1; c - 1 });
            let b = *map.entry(next::<String>(&mut it)).or_insert({ c += 1; c - 1 });
            writeln!(so, "{}", union_size(&mut root, &mut rank, &mut size, a, b))?;
        }
    }

    Ok(())
}
