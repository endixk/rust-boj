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

use std::collections::HashSet;
fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<i32>(&mut it);
    let mut set = HashSet::new();
    for _ in 0..n {
        let (i, j) = (next::<i32>(&mut it), next::<i32>(&mut it));
        if i == 0 { set.insert((0, if j > 0 { 1 } else { -1 })); continue; }
        if j == 0 { set.insert((if i > 0 { 1 } else { -1 }, 0)); continue; }
        let g = gcd(i.abs(), j.abs());
        set.insert((i / g, j / g));
    }
    writeln!(so, "{}", set.len()).ok();
}