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

    let (n, m, q) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let mut r = vec![0; n];
    let mut c = vec![0; m];
    for _ in 0..q {
        let (x, i, v) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<i32>(&mut it));
        if x == 1 { r[i - 1] += v; }
        else { c[i - 1] += v; }
    }
    for i in 0..n {
        for j in 0..m {
            write!(so, "{} ", r[i] + c[j]).unwrap();
        }
        writeln!(so).unwrap();
    }
}
