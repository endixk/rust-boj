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
    let mut a = vec![0; m];
    for _ in 0..n {
        for i in 0..m {
            a[i] += next::<usize>(&mut it);
        }
    }

    let k = next::<usize>(&mut it);
    let mut ans = a.iter().take(k).sum::<usize>();
    let mut sum = ans;
    for i in k..m {
        sum += a[i] - a[i - k];
        ans = ans.max(sum);
    }
    writeln!(so, "{}", ans).unwrap();
}