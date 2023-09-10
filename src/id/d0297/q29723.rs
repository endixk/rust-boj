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

use std::collections::HashMap;
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m, k) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let mut map = HashMap::new();
    for _ in 0..n {
        let (s, x) = (next::<String>(&mut it), next::<usize>(&mut it));
        map.insert(s, x);
    }

    let mut a = 0;
    for _ in 0..k {
        let s = next::<String>(&mut it);
        a += map.get(&s).unwrap();
        map.remove(&s);
    }
    let mut v = map.iter().map(|(_, &x)| x).collect::<Vec<_>>();
    v.sort_unstable_by(|a, b| b.cmp(a));

    let mut b = a;
    let x = v.len();
    for i in 0..m-k {
        a += v[i];
        b += v[x-1-i];
    }
    writeln!(so, "{} {}", b, a).ok();
}