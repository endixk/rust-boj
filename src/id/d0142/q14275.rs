// BOJ 14275 [Coloring a Rectangle]
// Supported by GitHub Copilot

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

const MOD: i32 = 1_000_000_007;
#[inline]
fn get(bit: usize, m: usize) -> Vec<u8> {
    (0..m).map(|i| ((bit / 3usize.pow(i as u32)) % 3) as u8).collect::<Vec<_>>()
}
// 0: empty, 1: odd-adjacent, 2: even-adjacent
fn valid(bit: &[u8], pbit: &[u8], m: usize) -> bool {
    for i in 0..m {
        if bit[i] == 0 {
            if pbit[i] == 1 { return false; }
        } else if bit[i] == 1 {
            if pbit[i] == 2 { return false; }
            if !((pbit[i] == 1) ^ (i > 0 && bit[i-1] > 0) ^ (i < m-1 && bit[i+1] > 0)) { return false; }
        } else if bit[i] == 2 {
            if pbit[i] == 2 { return false; }
            if (pbit[i] == 1) ^ (i > 0 && bit[i-1] > 0) ^ (i < m-1 && bit[i+1] > 0) { return false; }
        }
    }
    true
}
fn go(dp: &mut Vec<Vec<i32>>, stats: &Vec<Vec<u8>>, pairs: &Vec<Vec<usize>>, ends: &[bool],
      i: usize, pbit: usize, n: usize, m: usize) -> i32 {
    if i == n { return if ends[pbit] { 1 } else { 0 }; }
    if dp[i][pbit] >= 0 {
        return dp[i][pbit];
    }
    let mut ret = 0;
    for &bit in &pairs[pbit] {
        ret = (ret + go(dp, stats, pairs, ends, i+1, bit, n, m)) % MOD;
    }
    dp[i][pbit] = ret;
    ret
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let p = 3usize.pow(m as u32);
    let mut stats = vec![vec![]; p];
    for bit in 0..p {
        stats[bit] = get(bit, m);
    }
    let mut pairs = vec![vec![]; p];
    for b1 in 0..p {
        for b2 in 0..p {
            if valid(&stats[b1], &stats[b2], m) {
                pairs[b2].push(b1);
            }
        }
    }
    let mut ends = vec![false; p];
    for bit in 0..p {
        ends[bit] = stats[bit].iter().all(|&x| x != 1);
    }
    let mut dp = vec![vec![-1; p]; n];
    writeln!(so, "{}", go(&mut dp, &stats, &pairs, &ends, 0, 0, n, m)).ok();
}
