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

    let (n, t) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut r = 1;
    let mut map = HashMap::new();
    for _ in 0..t {
        let (a, b) = (next::<String>(&mut it), next::<String>(&mut it));
        let b = if b.contains("%") {
            let p = b.replace("%", "").parse::<usize>().unwrap();
            (n - r + 1) * p / 100
        } else {
            b.parse::<usize>().unwrap().min(n - r + 1)
        };
        if r + b > n + 1 { writeln!(so, "Invalid System").ok(); return; }
        if b > 0 { map.insert(a.clone(), (r, r + b - 1)); }

        let c = b / 4 + if b % 4 == 0 { 0 } else { 1 };
        let mut x = 0;
        for i in 1..5 {
            if x >= b { break; }
            let (s, e) = (r + x, r + if x + c >= b { b } else { x + c } - 1);
            // println!("{}{} {} {}", a, i, s, e);
            map.insert(format!("{}{}", a, i), (s, e));
            x += c;
        }
        r += b;
    }
    if r != n + 1 { writeln!(so, "Invalid System").ok(); return; }

    let q = next::<String>(&mut it);
    if !map.contains_key(&q) { writeln!(so, "Liar").ok(); return; }
    else {
        let (s, e) = map.get(&q).unwrap();
        writeln!(so, "{} {}", s, e).ok();
    }
}