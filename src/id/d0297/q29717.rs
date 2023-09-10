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

    for _ in 0..next(&mut it) {
        let n = next::<i64>(&mut it);
        let exp = n*(n+1)/2;

        let (mut l, mut r) = (1, 1000000000);
        while l < r {
            let m = (l+r)/2;
            if m*(m-1) > exp { r = m; } else { l = m+1; }
        }
        writeln!(so, "{}", l-1).ok();
    }
}