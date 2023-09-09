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

    let (n, m, k) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let h = (0..n).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
    let mut c = vec![0; 201];
    for x in h { c[x] += 1; }

    let l = (0..m).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
    let mut f = vec![0; 201];
    for i in 1..=200 {
        let (mut mx, mut mj) = (0, 0);
        for j in 0..m {
            if l[j] <= i && mx < l[j] { mx = l[j]; mj = j; }
        }
        f[i] = mj + 1;
    }

    let (mut ans, mut ak) = (0, 2);
    for a in 2..=m {
        let mut cnt = 0;
        for i in 1..=200 {
            let mut lv = i;
            for _ in 0..k {
                if lv >= 200 { break; }
                let go = if a > f[lv] { a - f[lv] } else { f[lv] - a };
                if f[lv] - 1 > go { cnt += c[i] * (f[lv] - go - 1); }
                lv += 1;
            }
        }
        if ans < cnt { ans = cnt; ak = a; }
    }

    writeln!(so, "1 {}", ak).ok();
    writeln!(so, "{}", ans).ok();
}