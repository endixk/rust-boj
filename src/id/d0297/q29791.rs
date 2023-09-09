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

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut a = (0..n).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
    let mut b = (0..m).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
    a.sort_unstable();
    b.sort_unstable();
    let (mut x, mut y) = (a[0], b[0]);
    let (mut p, mut q) = (1, 1);
    for &k in a.iter().skip(1) {
        if k >= x + 100 {
            p += 1; x = k;
        }
    }
    for &k in b.iter().skip(1) {
        if k >= y + 360 {
            q += 1; y = k;
        }
    }
    writeln!(so, "{} {}", p, q).unwrap();
}