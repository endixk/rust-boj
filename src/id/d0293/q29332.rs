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

    let (mut xmax, mut xmin, mut ymax, mut ymin) = (i64::MAX, i64::MIN, i64::MAX, i64::MIN);
    for _ in 0..next(&mut it) {
        let (x, y, d) = (next::<i64>(&mut it), next::<i64>(&mut it), next::<char>(&mut it));
        match d {
            'D' if y-1 < ymax => { ymax = y-1; },
            'U' if y+1 > ymin => { ymin = y+1; },
            'R' if x+1 > xmin => { xmin = x+1; },
            'L' if x-1 < xmax => { xmax = x-1; },
            _ => {},
        }
    }
    if xmax == i64::MAX || xmin == i64::MIN || ymax == i64::MAX || ymin == i64::MIN {
        writeln!(so, "Infinity").unwrap();
    } else {
        writeln!(so, "{}", (xmax-xmin+1)*(ymax-ymin+1)).unwrap();
    }
}