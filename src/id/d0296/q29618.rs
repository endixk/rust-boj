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

use std::collections::BinaryHeap;
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, q) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let qv = (0..q).map(|_| (
        next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it)
    )).collect::<Vec<_>>();

    let mut lv = vec![vec![]; n+1];
    for (i, &(l, r, v)) in qv.iter().enumerate() {
        lv[l].push((q-i, r, v));
    }

    let mut pq = BinaryHeap::<(usize, usize, usize)>::new();
    for i in 1..=n {
        while let Some((j, r, v)) = pq.pop() {
            if r < i { continue; }
            pq.push((j, r, v));
            break;
        }
        for &(j, r, v) in &lv[i] {
            pq.push((j, r, v));
        }
        write!(so, "{} ", if pq.is_empty() { 0 } else { pq.peek().unwrap().2 }).unwrap();
    }
    writeln!(so, "").unwrap();
}