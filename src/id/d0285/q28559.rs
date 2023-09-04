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

fn go(v: &[(i32, i32)], n: usize, d: i32, k: i32) -> bool {
    let mut x = v[0].1;
    for i in 1..n {
        if (v[i].0 - v[i-1].0).abs() > d { x = 0; }
        x += v[i].1;
        if x >= k { return true; }
    }
    false
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, k) = (next::<usize>(&mut it), next::<i32>(&mut it));
    let mut v = vec![(0, 0); n];
    (0..n).for_each(|i| v[i].0 = next::<i32>(&mut it));
    (0..n).for_each(|i| v[i].1 = next::<i32>(&mut it));
    v.sort_unstable();

    let (mut l, mut r) = (0, i32::MAX);
    while l < r {
        let m = (l + r) / 2;
        if go(&v, n, m, k) { r = m; } else { l = m + 1; }
    }
    writeln!(so, "{}", l).ok();
}
