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

    let (n, _, k) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let mut ans = 0;
    for _ in 0..n {
        let s = next::<String>(&mut it);
        let mut p = 0;
        for c in s.chars() {
            if c == '0' { p += 1; }
            else {
                ans += if p >= k { p - k + 1 } else { 0 };
                p = 0;
            }
        }
        ans += if p >= k { p - k + 1 } else { 0 };
    }
    writeln!(so, "{}", ans).unwrap();
}