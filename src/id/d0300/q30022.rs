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

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, a, _) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let (mut p, mut q) = (std::collections::BinaryHeap::new(), 0);
    for _ in 0..n {
        let (x, y) = (next::<i64>(&mut it), next::<i64>(&mut it));
        p.push(y-x);
        q += y;
    }
    for _ in 0..a { q -= p.pop().unwrap(); }
    writeln!(so, "{}", q)?;

    Ok(())
}