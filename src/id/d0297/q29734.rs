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

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let (t, s) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let z = (n-1)/8*(8+s)+(n-1)%8+1;
    let d = (m-1)/8*(8+2*t+s)+t+(m-1)%8+1;
    if z < d {
        writeln!(so, "Zip\n{}", z).unwrap();
    } else {
        writeln!(so, "Dok\n{}", d).unwrap();
    }
}