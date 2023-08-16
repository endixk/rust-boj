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
    let mut a = (0..n).map(|_| next::<i64>(&mut it)).collect::<Vec<_>>();
    a.sort_unstable();

    let (mut x, mut m) = (0, 0);
    for i in 0..n {
        while a[i] - a[x] >= n as i64 {
            x += 1;
        }
        m = m.max(i - x + 1);
    }
    println!("{}", n - m);
}
