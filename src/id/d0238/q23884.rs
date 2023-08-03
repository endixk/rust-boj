// BOJ 23883 [Selection Sort 4]
// Supported by GitHub Copilot

use std::io::{self, Read};
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
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, k) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut v = (0..n as u32).map(|i| (next::<u32>(&mut it), i)).collect::<Vec<_>>();

    v.sort_unstable();
    let map = v.iter().map(|&(x, _)| x).collect::<Vec<_>>();
    let mut val = vec![0; n];
    let mut loc = vec![0; n];
    v.into_iter().enumerate().for_each(|(j, (_, i))| {
        val[i as usize] = j;
        loc[j] = i as usize;
    });

    let mut cnt = 0;
    let mut buf = String::new();
    for i in (0..n).rev() {
        if loc[i] == i { continue; }
        cnt += 1;
        loc[val[i]] = loc[i];
        val[loc[i]] = val[i];
        loc[i] = i;
        val[i] = i;
        if cnt == k {
            val.into_iter().for_each(|i| buf.push_str(&format!("{} ", map[i])));
            println!("{}", buf);
            return;
        }
    }
    println!("-1");
}
