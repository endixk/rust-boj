// BOJ 5369 [Binary Search Tree]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read<T>(si: &mut T) -> String where T: Read {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

fn solve<T>(so: &mut T, v: &Vec<u32>, l: usize, r: usize) where T: Write {
    if l > r { return; }
    if l == r { writeln!(so, "{}", v[l]).ok(); return; }

    let mut p = l+1;
    while p <= r && v[p] < v[l] { p += 1; }
    solve(so, v, l+1, p-1);
    solve(so, v, p, r);
    writeln!(so, "{}", v[l]).ok();
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);

    let v = s.split_ascii_whitespace().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    solve(&mut so, &v, 0, v.len()-1);
}
