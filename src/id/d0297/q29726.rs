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
    let a = (0..n).map(|_| next::<i32>(&mut it)).collect::<Vec<_>>();
    let mut pm = vec![0; n];
    pm[n-1] = a[n-1];
    for i in 1..=m {
        pm[n-i-1] = pm[n-i].max(a[n-i-1]);
    }

    let mut ans = -9999999;
    for i in 0..m {
        if ans < pm[n-m+i-1] - a[i] { ans = pm[n-m+i-1] - a[i]; }
    }
    writeln!(so, "{}", ans).ok();
}