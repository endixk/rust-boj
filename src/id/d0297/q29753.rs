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

    let mut map = HashMap::new();
    map.insert("A+", 450);
    map.insert("A0", 400);
    map.insert("B+", 350);
    map.insert("B0", 300);
    map.insert("C+", 250);
    map.insert("C0", 200);
    map.insert("D+", 150);
    map.insert("D0", 100);
    map.insert("F", 0);

    let (n, x) = (next::<usize>(&mut it), next::<String>(&mut it));
    let x = &x[0..1].parse::<i32>().unwrap() * 100 + &x[2..].parse::<i32>().unwrap();
    let (mut m, mut f) = (0, 0);
    for _ in 1..n {
        let (c, g) = (next::<i32>(&mut it), next::<String>(&mut it));
        m += c;
        f += c * map.get(&g[..]).unwrap();
    }

    let c = next::<i32>(&mut it);
    let (mut r, mut d) = ("impossible", 500);
    for key in map.keys() {
        if (f + c * map.get(key).unwrap()) / (m + c) > x {
            if d > *map.get(key).unwrap() {
                r = key;
                d = *map.get(key).unwrap();
            }
        }
    }
    writeln!(so, "{}", r).unwrap();
}