// BOJ 30503 [Quadrat Sampling (Easy)]
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

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut a = (0..n).map(|_| next::<i32>(&mut it)).collect::<Vec<_>>();
    for _ in 0..next(&mut it) {
        match next::<u8>(&mut it) {
            1 => {
                let (l, r, k) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<i32>(&mut it));
                writeln!(so, "{}", a[l-1..r].iter().filter(|&&x| x == k).count())?;
            },
            _ => {
                let (l, r) = (next::<usize>(&mut it), next::<usize>(&mut it));
                a[l-1..r].iter_mut().for_each(|x| *x = 0);
            }
        }
    }

    Ok(())
}
