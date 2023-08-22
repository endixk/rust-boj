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

    let n = next::<usize>(&mut it);
    let a = (0..n).map(|_| next::<i64>(&mut it)).collect::<Vec<_>>();

    let mut sum = 0;
    let mut v = Vec::new();
    for x in a {
        sum += x;
        if sum > 0 {
            v.push(sum);
            sum = 0;
        }
    }

    if v.len() == 0 {
        writeln!(so, "0").unwrap();
        return;
    }
    let (mut a, mut b) = (v[0], 0);
    for &x in v.iter().skip(1) {
        if b + x > a {
            (a, b) = (b + x, a);
        } else {
            b += x;
        }
    }
    writeln!(so, "{}", a).unwrap();
}