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

fn sieve(n: usize) -> Vec<usize> {
    let mut isp = vec![true; n+1];
    isp[0] = false; isp[1] = false;
    for i in 2..=n {
        if i*i > n { break; }
        if isp[i] {
            let mut j = i+i;
            while j <= n {
                isp[j] = false;
                j += i;
            }
        }
    }
    isp.iter().enumerate().filter(|&(_, &b)| b).map(|(i, _)| i).collect()
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let p = sieve(n+10000);
    let (mut b, mut s, mut x) = (0, 0, 0);
    for v in p {
        if n < v { b += n - x; break; }
        let c = v - x;
        if c > 2 { b += c - 2; s += 2; }
        else { s += c; }
        x = v;
    }
    writeln!(so, "{} {}", b, s).unwrap();
}