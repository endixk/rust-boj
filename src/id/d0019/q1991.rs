// BOJ 1991 [Tree Traversal]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

fn order(adj: &[[u8; 2]], i: usize, ord: i8) -> String {
    let mut s = String::new();
    if ord == -1 {
        s.push((i as u8 + b'A') as char);
    }
    if adj[i][0] != 0 {
        s.push_str(&order(adj, adj[i][0] as usize, ord));
    }
    if ord == 0 {
        s.push((i as u8 + b'A') as char);
    }
    if adj[i][1] != 0 {
        s.push_str(&order(adj, adj[i][1] as usize, ord));
    }
    if ord == 1 {
        s.push((i as u8 + b'A') as char);
    }
    s
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let mut adj = [[0u8, 0u8]; 26];
    for _ in 0..it.by_ref().next().unwrap().parse().unwrap() {
        let a = (it.next().unwrap().as_bytes()[0] - b'A') as usize;
        let c = it.next().unwrap().as_bytes()[0];
        if c != b'.' {
            adj[a][0] = c - b'A';
        }
        let c = it.next().unwrap().as_bytes()[0];
        if c != b'.' {
            adj[a][1] = c - b'A';
        }
    }

    writeln!(so, "{}", order(&adj, 0, -1)).unwrap();
    writeln!(so, "{}", order(&adj, 0, 0)).unwrap();
    writeln!(so, "{}", order(&adj, 0, 1)).unwrap();
}
