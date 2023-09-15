// BOJ 11729 [Hanoi Tower Move Sequence]
// Supported by GitHub Copilot

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

fn hanoi(src: u8, tmp: u8, dst: u8, n: u8, so: &mut dyn Write) {
    if n == 1 { writeln!(so, "{} {}", src, dst).ok(); return; }
    hanoi(src, dst, tmp, n-1, so);
    writeln!(so, "{} {}", src, dst).ok();
    hanoi(tmp, src, dst, n-1, so);
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<u8>(&mut it);
    writeln!(so, "{}", (1 << n) - 1).ok();
    hanoi(1, 2, 3, n, &mut so);
}
