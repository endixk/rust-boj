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

const V: usize = 0x3f3f3f3f;
fn go(dp: &mut Vec<Vec<usize>>, a: &[usize], cur: usize, dir: usize, n: usize, t: usize) -> usize {
    if cur == n-1 { return t; }
    if a[cur] == 0 { return 0; }
    if dp[cur][dir] != V { return dp[cur][dir]; }

    let mut ret = 0;
    if dir == 0 {
        if cur + a[cur] < n {
            ret = ret.max(go(dp, a, cur+a[cur], 0, n, t+1));
        }
        if a[cur] <= cur {
            ret = ret.max(go(dp, a, cur-a[cur], 1, n, t+1));
        }
    } else if dir == 1 {
        if a[cur] <= cur {
            ret = ret.max(go(dp, a, cur-a[cur], 1, n, t+1));
        }
        if cur + a[cur] < n {
            ret = ret.max(go(dp, a, cur+a[cur], 2, n, t+1));
        }
    } else {
        if cur + a[cur] < n {
            ret = ret.max(go(dp, a, cur+a[cur], 2, n, t+1));
        }
    }
    dp[cur][dir] = ret;
    ret
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let a = (0..n).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();

    let mut dp = vec![vec![V; 3]; n];
    let ans = go(&mut dp, &a, 0, 0, n, 0);
    if ans == 0 { writeln!(so, "-1")?; }
    else { writeln!(so, "{}", ans)?; }

    Ok(())
}