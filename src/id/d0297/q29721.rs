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
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, k) = (next::<i32>(&mut it), next::<usize>(&mut it));
    let mut ss = HashSet::new();
    for _ in 0..k {
        ss.insert((next::<i32>(&mut it), next::<i32>(&mut it)));
    }

    let mut sd = HashSet::new();
    for &(i, j) in ss.iter() {
        if i + 2 <= n && !ss.contains(&(i + 2, j)) { sd.insert((i + 2, j)); }
        if j + 2 <= n && !ss.contains(&(i, j + 2)) { sd.insert((i, j + 2)); }
        if i - 2 >= 1 && !ss.contains(&(i - 2, j)) { sd.insert((i - 2, j)); }
        if j - 2 >= 1 && !ss.contains(&(i, j - 2)) { sd.insert((i, j - 2)); }
    }

    writeln!(so, "{}", sd.len()).ok();
}