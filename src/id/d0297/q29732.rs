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

    let (_, m, k) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<i32>(&mut it));
    let s = next::<String>(&mut it);
    let v = s.chars().map(|c| c =='R').collect::<Vec<_>>();
    let mut locs = v.iter().enumerate().filter(|&(_, &b)| b).map(|(i, _)| i as i32).collect::<Vec<_>>();
    locs.push(99999);

    let (mut a, mut b, mut j) = (-99999, locs[0], 0);
    let mut c = 0;
    for (i, &r) in v.iter().enumerate() {
        if r {
            c += 1;
            a = b;
            j += 1;
            b = locs[j];
        } else {
            if i as i32 - a <= k { c += 1; }
            else if b - i as i32 <= k { c += 1; }
        }
    }
    writeln!(so, "{}", if c > m { "No" } else { "Yes" }).ok();
}