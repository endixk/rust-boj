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

fn go(d: usize, bit: usize, k: usize, p: &[usize], q: &[usize]) -> usize {
    let (mut t, mut r) = (0, 0);
    for b in 0..k {
        if bit & (1 << b) != 0 {
            t += (p[b] / d) + (if p[b] % d == 0 { 0 } else { 1 });
            if t > 900 { return 0; }
            r += q[b];
        }
    }
    r
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m, k) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let d = (0..n).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
    let (mut p, mut q) = (vec![0; k], vec![0; k]);
    for i in 0..k {
        p[i] = next::<usize>(&mut it);
        q[i] = next::<usize>(&mut it);
    }

    let mut r = vec![0; n];
    for i in 0..n {
        for bit in 1..1<<k {
            r[i] = r[i].max(go(d[i], bit, k, &p, &q));
        }
    }
    r.sort_unstable();
    writeln!(so, "{}", (1..=m).map(|i| r[n-i]).sum::<usize>()).unwrap();
}