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

    let (a, b) = (next::<i32>(&mut it), next::<i32>(&mut it));
    let (k, x) = (next::<i32>(&mut it), next::<i32>(&mut it));
    let a = a.max(k-x);
    let b = b.min(k+x);
    writeln!(so, "{}", if a <= b { (b-a+1).to_string() }  else { "IMPOSSIBLE".to_string() }).unwrap();
}