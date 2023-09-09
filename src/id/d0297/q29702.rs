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

fn bit(n: i64) -> (i64, i64) {
    let mut k = 0;
    let mut m = n;
    while m > 0 {
        k += 1;
        m >>= 1;
    }
    (k, n - (1 << (k - 1)) + 1)
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    for _ in 0..next(&mut it) {
        let mut n = next::<i64>(&mut it);
        while n > 0 {
            let (k, q) = bit(n);
            writeln!(so, "{}{:0>18}", k, q).ok();
            n >>= 1;
        }
    }
}