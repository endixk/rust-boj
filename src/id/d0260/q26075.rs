// BOJ 26075 [Don't Cross the Line]
// Supported by GitHub Copilot

use std::io::{self, Read};

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
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, _) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let (s, t) = (next::<String>(&mut it), next::<String>(&mut it));
    let (s, t) = (s.as_bytes(), t.as_bytes());

    let mut cnt = 0;
    let (mut i, mut j) = (0, 0);
    for _ in 0..n {
        while s[i] == b'1' { i += 1; }
        while t[j] == b'1' { j += 1; }
        cnt += if i > j { i - j } else { j - i };
        i += 1;
        j += 1;
    }

    println!("{}",
        if cnt % 2 == 0 {
            (cnt / 2).pow(2) * 2
        } else {
            (cnt / 2).pow(2) + (cnt / 2 + 1).pow(2)
        }
    );
}
