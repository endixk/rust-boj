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

    let (n, m, k) = (next::<i32>(&mut it), next::<i32>(&mut it), next::<i32>(&mut it));
    let a = n - m*k;
    let b = n - m*k + m - 1;
    writeln!(so, "{} {}", if a < 0 { 0 } else { a }, if b < 0 { 0 } else { b }).unwrap();
}