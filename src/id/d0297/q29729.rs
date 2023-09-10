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

    let (mut k, n, m) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let (mut sz, mut ms) = (0, 0);
    for _ in 0..n+m {
        if next::<u8>(&mut it) == 1 {
            sz += 1;
            if sz > ms { ms = sz; }
        } else { sz -= 1; }
    }

    while k < ms { k <<= 1; }
    writeln!(so, "{}", k).ok();
}