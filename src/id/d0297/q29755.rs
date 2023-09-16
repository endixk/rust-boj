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
    let mut b = (0..n).map(|_| next::<i32>(&mut it)).collect::<Vec<_>>();
    b.push(-9999999); b.push(9999999);
    b.sort_unstable();

    let mut ans = 0;
    for _ in 0..m {
        let (a, w) = (next::<i32>(&mut it), next::<i32>(&mut it));
        match b.binary_search(&a) {
            Ok(_) => { continue; }
            Err(x) => {
                let d = (b[x] - a).abs().min((b[x-1] - a).abs());
                if ans < d * w { ans = d * w; }
            }
        }
    }
    writeln!(so, "{}", ans).unwrap();
}