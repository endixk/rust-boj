// BOJ 17103 [Goldbach Partition]
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

fn sieve(n: usize) -> (Vec<bool>, Vec<usize>) {
    let mut isp = vec![true; n + 1];
    let mut p = vec![];
    isp[0] = false; isp[1] = false;
    for i in 2..=n {
        if isp[i] {
            p.push(i);
            for j in (i * i..=n).step_by(i) {
                isp[j] = false;
            }
        }
    }
    (isp, p)
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (isp, p) = sieve(1000000);
    for _ in 0..next(&mut it) {
        let n = next::<usize>(&mut it);
        let mut cnt = 0;
        for &x in &p {
            if x > n / 2 { break; }
            if isp[n - x] { cnt += 1; }
        }
        writeln!(so, "{}", cnt).unwrap();
    }
}
