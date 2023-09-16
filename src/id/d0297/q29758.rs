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

    for _ in 0..next(&mut it) {
        let (n, l) = (next::<usize>(&mut it), next::<usize>(&mut it));
        if l == 1 { writeln!(so, "{}", if n == 1 { "1" } else { "0" }).unwrap(); continue; }

        let mut dp = vec![0; 270];
        for i in 1..270 { if i*i < n { dp[i] = 1; } }
        for _ in 2..l {
            let mut tp = vec![0; 270];
            for i in 1..270 {
                for j in i*i+1..270 {
                    tp[i] += dp[j];
                }
            }
            dp = tp;
        }

        writeln!(so, "{}", dp[1]).unwrap();
    }
}