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

    let v = next::<String>(&mut it).split(":").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();
    let st = v[0]*60 + v[1];
    let v = next::<String>(&mut it).split(":").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();
    let et = v[0]*60 + v[1];

    let (n, t) = (next::<u32>(&mut it), next::<u32>(&mut it));
    let (mut d, mut x) = (0, st);
    for _ in 0..=n {
        if x + t >= et { d += 1; x = st; }
        x += t;
    }
    writeln!(so, "{}", d).ok();
    writeln!(so, "{:02}:{:02}", x/60, x%60).ok();
}