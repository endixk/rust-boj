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

    let (mut a, mut b) = (0, 0);
    for _ in 0..next(&mut it) {
        let (t, w, h, l) = (
            next::<char>(&mut it),
            next::<i64>(&mut it), next::<i64>(&mut it), next::<i64>(&mut it));
        if t == 'B' { a += 6000; }
        else {
            let k = (w / 12) * (h / 12) * (l / 12);
            a += 1000 + 500 * k;
            b += 4000 * k;
        }
    }
    writeln!(so, "{}\n{}", a, b).unwrap();
}