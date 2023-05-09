// BOJ 9019 [DSLR]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

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

fn df(n: u16) -> u16 { (n << 1) % 10000 }
fn sf(n: u16) -> u16 { if n == 0 { 9999 } else { n - 1 } }
fn lf(n: u16) -> u16 { n / 1000 + (n % 1000) * 10 }
fn rf(n: u16) -> u16 { n % 10 * 1000 + n / 10 }
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    for _ in 0..next(&mut it ){
        let (a, b) = (next::<u16>(&mut it), next::<u16>(&mut it));
        let mut vis = [false; 10000];
        let mut trk = [0; 10000];
        let mut cmd = ['\0'; 10000];

        let mut q = std::collections::VecDeque::new();
        vis[a as usize] = true;
        q.push_back(a);
        while let Some(x) = q.pop_front() {
            if x == b { break; }
            let (d, s, l, r) = (df(x), sf(x), lf(x), rf(x));
            if !vis[d as usize] {
                vis[d as usize] = true; trk[d as usize] = x; cmd[d as usize] = 'D'; q.push_back(d);
            }
            if !vis[s as usize] {
                vis[s as usize] = true; trk[s as usize] = x; cmd[s as usize] = 'S'; q.push_back(s);
            }
            if !vis[l as usize] {
                vis[l as usize] = true; trk[l as usize] = x; cmd[l as usize] = 'L'; q.push_back(l);
            }
            if !vis[r as usize] {
                vis[r as usize] = true; trk[r as usize] = x; cmd[r as usize] = 'R'; q.push_back(r);
            }
        }

        let mut ans = String::new();
        let mut x = b;
        while x != a {
            ans.push(cmd[x as usize]);
            x = trk[x as usize];
        }
        writeln!(so, "{}", ans.chars().rev().collect::<String>()).ok();
    }
}
