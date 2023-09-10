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

    for t in 1..=next(&mut it) {
        match next::<i32>(&mut it) {
            x if x > 4500 => { writeln!(so, "Case #{}: Round 1", t).unwrap(); }
            x if x > 1000 => { writeln!(so, "Case #{}: Round 2", t).unwrap(); }
            x if x > 25 => { writeln!(so, "Case #{}: Round 3", t).unwrap(); }
            _ => writeln!(so, "Case #{}: World Finals", t).unwrap(),
        }
    }
}