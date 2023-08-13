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
    let mut a = (0..n).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
    a.sort_unstable_by(|x, y| y.cmp(x));

    let (mut sum, mut lvl) = (0, 0);
    for i in 0..42.min(n) {
        sum += a[i];
        lvl += match a[i] {
            x if x > 249 => 5,
            x if x > 199 => 4,
            x if x > 139 => 3,
            x if x > 99 => 2,
            x if x > 59 => 1,
            _ => 0,
        }
    }
    println!("{} {}", sum, lvl);
}