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

    let n = next::<usize>(&mut it);
    let mut cnt = vec![0; 100001];
    for _ in 0..n {
        cnt[next::<usize>(&mut it)] += 1;
    }

    for i in 1..=100000 {
        if cnt[i] == 0 { continue; }
        for j in (i*2..=100000).step_by(i) {
            cnt[j] += cnt[i];
        }
    }

    for _ in 0..next(&mut it) {
        write!(so, "{} ", cnt[next::<usize>(&mut it)]).unwrap();
    }
    writeln!(so).unwrap();
}
