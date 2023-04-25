// BOJ 1764 [Unheard and Unseen]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let v: Vec<usize> = it.by_ref().take(2).map(|x| x.parse().unwrap()).collect();

    let mut set = HashSet::new();
    it.by_ref().take(v[0]).for_each(|x| { set.insert(x); });

    let mut heap = BinaryHeap::new();
    it.by_ref().take(v[1]).filter(|&x| set.contains(x)).for_each(|x| heap.push(Reverse(x)));
    writeln!(so, "{}", heap.len()).unwrap();
    while let Some(Reverse(x)) = heap.pop() {
        writeln!(so, "{}", x).unwrap();
    }
}
