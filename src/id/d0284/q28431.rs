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

    let mut ans = 0;
    let mut ex = [false; 10];
    for _ in 0..5 {
        let x = next::<usize>(&mut it);
        if ex[x] { ans -= x; ex[x] = false; }
        else { ans += x; ex[x] = true; }
    }
    writeln!(so, "{}", ans).ok();
}