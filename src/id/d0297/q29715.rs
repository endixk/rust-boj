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

fn ncr(n: usize, r: usize) -> usize {
    let mut ans = 1;
    for i in 0..r {
        ans *= n - i;
        ans /= i + 1;
    }
    ans
}
fn fact(n: usize) -> usize {
    let mut ans = 1;
    for i in 1..=n {
        ans *= i;
    }
    ans
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let (x, y) = (next::<usize>(&mut it), next::<usize>(&mut it));

    let (mut p, mut q) = (n, 9);
    for _ in 0..m {
        let (a, _) = (next::<usize>(&mut it), next::<usize>(&mut it));
        if a == 0 { q -= 1; }
        else { p -= 1; q -= 1; }
    }

    let c = ncr(q, n-m) * fact(p);
    writeln!(so, "{}", c * x + (c - 1) / 3 * y).unwrap();
}