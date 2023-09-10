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
    let v = s.split("-").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let mut x = v[0] * 360 + (v[1]-1) * 30 + (v[2]-1);
    x += next::<i32>(&mut it);
    writeln!(so, "{}-{:0>2}-{:0>2}", x / 360, x % 360 / 30 + 1, x % 30 + 1).unwrap();
}