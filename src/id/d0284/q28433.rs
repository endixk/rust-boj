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
        let n = next::<usize>(&mut it);
        let v = (0..n).map(|_| next::<i64>(&mut it)).collect::<Vec<_>>();

        let mut pf = vec![0; n];
        let mut cnt = 0;
        let mut sum = 0;
        for (i, &x) in v.iter().enumerate() {
            if x < 0 {
                if sum > 0 && sum + x <= 0 {
                    cnt += 1;
                    sum = x;
                } else { sum += x; }
            } else if x > 0 {
                if sum <= 0 && sum + x > 0 {
                    sum += x;
                } else {
                    cnt += if sum > 0 { 1 } else if sum == 0 { 0 } else { -1 };
                    sum = x;
                }
            }
            pf[i] = if sum > 0 { cnt + 1 } else if sum == 0 { cnt } else { cnt - 1 };
        }
        if pf[n-1] > 0 {
            writeln!(so, "YES").ok();
        } else {
            writeln!(so, "NO").ok();
        }
    }
}
