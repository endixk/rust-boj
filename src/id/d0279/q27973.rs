// BOJ 27973 [Lazy Evaluation]
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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (mut x, mut f) = (1, 1);
    for _ in 0..next(&mut it) {
        match next::<u8>(&mut it) {
            0 => x += next::<i64>(&mut it),
            1 => {
                let k = next::<i64>(&mut it);
                x *= k; f *= k;
            },
            2 => x += f * next::<i64>(&mut it),
            _ => writeln!(so, "{}", x).unwrap(),
        }
    }
}
