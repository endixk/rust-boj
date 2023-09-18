// BOJ 25201 [Board Flipping]
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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (mut rs, mut cs) = (vec![0; 100000], vec![0; 100000]);
    let (mut rd, mut cd) = (vec![0; 100000], vec![0; 100000]);
    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    (0..n).map(|_| (next::<usize>(&mut it), next::<usize>(&mut it)))
        .for_each(|(x, y)| { rs[x-1] += 1; cs[y-1] += 1; });
    (0..m).map(|_| (next::<usize>(&mut it), next::<usize>(&mut it)))
        .for_each(|(x, y)| { rd[x-1] += 1; cd[y-1] += 1; });
    for (i, j) in rs.into_iter().zip(rd.into_iter()) {
        if i%2 != j%2 { writeln!(so, "NO").ok(); return; }
    }
    for (i, j) in cs.into_iter().zip(cd.into_iter()) {
        if i%2 != j%2 { writeln!(so, "NO").ok(); return; }
    }
    writeln!(so, "YES").ok();
}
