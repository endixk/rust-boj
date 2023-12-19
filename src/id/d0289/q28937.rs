// BOJ 28937 [Dipper's Gift]
// Supported by GitHub Copilot

fn sieve(x: usize) -> Vec<usize> {
    let mut v = vec![true; x + 1];
    v[0] = false; v[1] = false;
    for i in 2..=x {
        if v[i] { for j in (i * 2..=x).step_by(i) { v[j] = false; } }
    }
    v.iter().enumerate().filter(|(_, &x)| x).map(|(i, _)| i).collect()
}
fn floyd(adj: &mut Vec<Vec<i64>>, n: usize) {
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                adj[i][j] = adj[i][j].min(adj[i][k] + adj[k][j]);
            }
        }
    }
}
pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());
    let s = next::<String>().chars().map(|x| x as usize - 97).collect::<Vec<_>>();

    const INF: i64 = 0x3f3f3f3f3f3f3f3f;
    let mut cost = vec![vec![INF; 26]; 26];
    for i in 0..26 { cost[i][i] = 0; }
    for _ in 0..m {
        let (a, b, c) = (next::<char>(), next::<char>(), next::<i64>());
        let (a, b) = (a as usize - 97, b as usize - 97);
        cost[a][b] = cost[a][b].min(c);
    }
    floyd(&mut cost, 26);

    let mut ans = INF;
    'a: for x in 0..26 {
        let mut c = 0;
        for i in 0..n {
            if cost[s[i]][x] == INF { continue 'a; }
            c += cost[s[i]][x];
        }
        ans = ans.min(c);
    }
    'x: for x in sieve(n) {
        if x*2 > n { break; }
        if n % x != 0 { continue; }
        let mut c = 0;
        for i in 0..n/x {
            let mut opt = INF;
            'j: for j in 0..26 {
                let mut tot = 0;
                for k in 0..x {
                    let idx = k*(n/x) + i;
                    if cost[s[idx]][j] == INF { continue 'j; }
                    tot += cost[s[idx]][j];
                }
                opt = opt.min(tot);
            }
            if opt == INF { continue 'x; }
            c += opt;
        }
        ans = ans.min(c);
    }

    println!("{}", if ans == INF { -1 } else { ans });
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug};
static mut BUF: String = String::new();
static mut IT: Option<SplitAsciiWhitespace> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut BUF).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}