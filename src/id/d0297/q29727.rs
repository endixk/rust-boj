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

    let n = next::<i64>(&mut it);
    let (xa, ya) = (next::<i64>(&mut it), next::<i64>(&mut it));
    let (xb, yb) = (next::<i64>(&mut it), next::<i64>(&mut it));

    let mut ans = (n+1)*n/2 * (n+1)*n/2;
    if xa == xb {
        let (ya, yb) = if ya > yb { (yb, ya) } else { (ya, yb) };
        let a = 0.max(ya + 1);
        let b = n.min(yb);
        let c = b - a + 1;
        if c > 0 { ans += c * (c-1) / 2 * (n+1); }
    } else if ya == yb {
        let (xa, xb) = if xa > xb { (xb, xa) } else { (xa, xb) };
        let a = 0.max(xa + 1);
        let b = n.min(xb);
        let c = b - a + 1;
        if c > 0 { ans += c * (c-1) / 2 * (n+1); }
    }

    writeln!(so, "{}", ans).ok();
}