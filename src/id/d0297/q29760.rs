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

    let (n, h, _) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let mut p = vec![vec![]; h+1];
    p[1].push(1);
    for _ in 0..n {
        let (i, j) = (next::<usize>(&mut it), next::<usize>(&mut it));
        p[i].push(j as i32);
    }
    for i in 1..=h { p[i].sort_unstable(); }

    let (mut l, mut r) = (1, 1);
    let (mut a, mut b) = (0, 0);
    let mut k = 0;
    for i in 1..=h {
        if p[i].is_empty() { continue; }
        let (x, y) = (p[i][0], *p[i].last().unwrap());
        (a, b) = (
            (i-k) as i32 * 100 + y-x + (a + (y - l).abs()).min(b + (y - r).abs()),
            (i-k) as i32 * 100 + y-x + (a + (x - l).abs()).min(b + (x - r).abs())
        );
        l = x; r = y; k = i;
    }
    writeln!(so, "{}", a.min(b) - 100).unwrap();
}