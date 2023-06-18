// BOJ 19541 [Epidemiologic Survey]
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

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let epi = (0..m).map(|_| {
        let k = next::<usize>(&mut it);
        (0..k).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    let pos = (0..n).map(|_| next::<usize>(&mut it) == 1).collect::<Vec<_>>();

    let mut sus = pos.clone();
    for e in epi.iter().rev() {
        if e.iter().any(|&i| !sus[i-1]) {
            e.iter().for_each(|&i| sus[i-1] = false);
        }
    }

    let mut sim = sus.clone();
    for e in epi {
        if e.iter().any(|&i| sim[i-1]) {
            e.iter().for_each(|&i| sim[i-1] = true);
        }
    }

    if pos == sim {
        writeln!(so, "YES").unwrap();
        sus.iter().for_each(|&b| write!(so, "{} ", b as u8).unwrap());
    } else {
        writeln!(so, "NO").unwrap();
    }
}
