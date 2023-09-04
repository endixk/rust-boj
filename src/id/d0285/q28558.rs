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

fn calc(t: u64, k: u64) -> u64 {
    let (p, x) = (t / k, t % k);
    k * p * (p + 1) / 2 + x * (p + 1)
}
fn go(t: u64, k: u64, a: &mut u64, b: &mut u64) {
    if t & 1 == 1 {
        let t = (t + 1) >> 1;
        *a += calc(t, k);
        *b += calc(t, k);
    } else {
        let t = t >> 1;
        *a += calc(t, k) + t / k + 1;
        *b += calc(t, k);
    }
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (a, b, t) = (next::<u64>(&mut it), next::<u64>(&mut it), next::<u64>(&mut it));
    let (mut ax, mut bx) = (0, 0);
    go(t, a, &mut ax, &mut bx);
    if t >= a { go(t-a, a, &mut ax, &mut bx); }
    go(t, b, &mut bx, &mut ax);
    if t >= b { go(t-b, b, &mut bx, &mut ax); }
    writeln!(so, "{} {}", ax, bx).ok();
}
