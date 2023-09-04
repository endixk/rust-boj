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

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}
fn vgcd(v: &[usize]) -> usize {
    v.iter().fold(v[0], |a, &b| gcd(a, b))
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, _) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let v = (0..n).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
    let g = vgcd(&v);
    write!(so, "{} ", v.iter().sum::<usize>() / g).unwrap();
    let w = v.iter().map(|&x| x / g).collect::<Vec<_>>();

    let v = (0..n).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
    let g = vgcd(&v);
    let v = v.iter().map(|&x| x / g).collect::<Vec<_>>();
    let mut f = 0;
    for (&i, &j) in w.iter().zip(v.iter()) {
        if j == 0 { continue; }
        let k = i / j + if i % j == 0 { 0 } else { 1 };
        if k > f { f = k; }
    }
    writeln!(so, "{}", v.iter().sum::<usize>() * f).unwrap();
}