// BOJ 29335 [Coloring]
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

use std::collections::BTreeMap;
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let mut map = BTreeMap::new();
    map.insert(1e10 as usize, (0usize, 0usize));
    let mut a = 0;
    for _ in 0..next(&mut it) {
        let (x, h) = (next::<usize>(&mut it), next::<usize>(&mut it));
        let s = map.range(h+1..).next().unwrap().1.1;
        if s >= x {
            writeln!(so, "{}", a).unwrap();
            continue;
        }
        a += (x - s) * h;

        let mut d = Vec::new();
        for (&k, &(i, j)) in map.range(..=h).rev() {
            if i >= x { break; }
            d.push(k);
            a -= (j - i) * k;
        }
        if let Some(k) = d.pop() {
            let (_, j) = map.remove(&k).unwrap();
            if j > x {
                a += (j - x) * k;
                map.insert(k, (x, j));
            }
        }
        d.iter().for_each(|k| { map.remove(k); });
        match map.get(&h) {
            Some(&(_, j)) => map.insert(h, (s, j)),
            None => map.insert(h, (s, x)),
        };

        writeln!(so, "{}", a).unwrap();
    }
}
