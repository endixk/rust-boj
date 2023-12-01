// BOJ 30788 [Sakura Reflection]
// Supported by GitHub Copilot

pub fn main() {
    let s = read();
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let a = (0..n).map(|_| 2 * next::<usize>(&mut it)).collect::<Vec<_>>();
    if n % 2 == 1 { println!("NO"); return; }

    let mut dp = vec![vec![false; 360]; n/2+1];
    let mut tr = vec![vec![n; 360]; n/2+1];
    dp[0][0] = true;
    for i in 0..n {
        for j in (0..n/2).rev() {
            for k in 0..360 {
                if dp[j][k] {
                    dp[j+1][(k+a[i])%360] = true;
                    tr[j+1][(k+a[i])%360] = tr[j+1][(k+a[i])%360].min(i);
                }
            }
        }
    }

    let mut r = a.iter().sum::<usize>() % 360 / 2;
    let mut v = vec![false; n];
    if dp[n/2][r] {
        let mut i = n/2;
        while i > 0 {
            v[tr[i][r]] = true;
            r = (r + 360 - a[tr[i][r]]) % 360;
            i -= 1;
        }
    } else if dp[n/2][r+180] {
        r += 180;
        let mut i = n/2;
        while i > 0 {
            v[tr[i][r]] = true;
            r = (r + 360 - a[tr[i][r]]) % 360;
            i -= 1;
        }
    } else { println!("NO"); return; }

    println!("YES");
    let a = v.iter().enumerate().filter(|(_, &b)| b).map(|(i, _)| i+1).collect::<Vec<_>>();
    let b = v.iter().enumerate().filter(|(_, &b)| !b).map(|(i, _)| i+1).collect::<Vec<_>>();
    for (i, j) in a.iter().zip(b.iter()) {
        print!("{} {} ", i, j);
    }
    println!();
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug};
fn read() -> String {
    let mut s = String::new();
    SI.with(|c| c.borrow_mut().read_to_string(&mut s).unwrap());
    s
}
fn next<T: FromStr>(it: &mut SplitAsciiWhitespace) -> T where <T as FromStr>::Err: Debug {
    it.next().unwrap().parse().unwrap()
}