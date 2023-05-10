// BOJ 17219 [Finding Password]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn read<T>(si: &mut T) -> String where T: Read {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m): (usize, usize) = (it.next().unwrap().parse().unwrap(), it.next().unwrap().parse().unwrap());
    let map = (0..n).fold(HashMap::new(), |mut map, _| {
        let (a, b) = (it.next().unwrap(), it.next().unwrap());
        map.insert(a, b); map
    });

    (0..m).for_each(|_| {
        let a = it.next().unwrap();
        writeln!(so, "{}", map.get(&a).unwrap()).ok();
    });
}
