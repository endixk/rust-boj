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

    let s = next::<String>(&mut it);
    let (mut f, mut k) = (0.0, 0);
    for c in s.chars() {
        match c {
            'A' => { f += 4.0; k += 1; }
            'B' => { f += 3.0; k += 1; }
            'C' => { f += 2.0; k += 1; }
            'D' => { f += 1.0; k += 1; }
            'F' => { k += 1; }
            '+' => { f += 0.5; }
            _ => {}
        }
    }
    writeln!(so, "{}", f / k as f64).unwrap();
}