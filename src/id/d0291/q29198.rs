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
    let mut v = (0..n).map(|_| next::<String>(&mut it).as_bytes().to_vec()).collect::<Vec<_>>();
    for x in v.iter_mut() { x.sort_unstable(); }
    v.sort_unstable();

    let mut ans = Vec::new();
    for i in 0..k {
        ans.extend(v[i].iter());
    }
    ans.sort_unstable();
    writeln!(so, "{}", String::from_utf8(ans).unwrap()).ok();
}