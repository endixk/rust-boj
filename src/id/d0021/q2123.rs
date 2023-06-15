// BOJ 2123 [Cow Acrobats]
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

    let n = next::<usize>(&mut it);
    let mut v = (0..n).map(|_|
        (next::<i64>(&mut it), next::<i64>(&mut it))
    ).collect::<Vec<_>>();
    v.sort_unstable_by(|a, b| (a.0 + a.1).cmp(&(b.0 + b.1)));

    let mut ans = -v[0].1;
    let mut wsum = v[0].0;
    for c in v.iter().skip(1) {
        ans = ans.max(wsum - c.1);
        wsum += c.0;
    }
    println!("{}", ans);
}
