// BOJ 3015 [PATRIK]
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

    let mut st = Vec::new();
    let mut ans = 0;
    for _ in 0..next(&mut it) {
        let mut h = (next::<i32>(&mut it), 1u64);
        while let Some((x, k)) = st.pop() {
            if x > h.0 {
                ans += 1;
                st.push((x, k));
                st.push(h);
                break;
            } else if x == h.0 {
                ans += k;
                h = (x, k + 1);
            } else {
                ans += k;
            }
        }
        if st.is_empty() { st.push(h); }
    }

    println!("{}", ans);
}
