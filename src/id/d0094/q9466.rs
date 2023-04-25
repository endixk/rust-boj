// BOJ 9466 [Term Project]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
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
    let t = next(&mut it);
    for _ in 0..t {
        let n = next(&mut it);
        let v = (0..n).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();

        let mut vis = vec![0; n];
        let mut cnt = 0;
        let mut ans = 0;
        for i in 0..n {
            if vis[i] > 0 { continue; }
            cnt += 1;
            vis[i] = cnt;

            let start = cnt;
            let mut j = v[i] - 1;
            while vis[j] == 0 {
                cnt += 1;
                vis[j] = cnt;
                j = v[j] - 1;
            }

            if vis[j] >= start {
                ans += cnt - vis[j] + 1;
            }
        }

        writeln!(so, "{}", n - ans).unwrap();
    }
}
