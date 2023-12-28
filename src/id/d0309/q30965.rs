// BOJ 30965 [Tour]
// Supported by GitHub Copilot

pub fn main() { read();
    let (n, q, m) = (next::<usize>(), next::<usize>(), next::<i64>());

    let a = (0..n).map(|_| next::<i64>() % m).collect::<Vec<_>>();
    let mut p = vec![vec![0; m as usize]; n+1];
    for i in 0..n {
        p[i+1] = p[i].clone();
        p[i+1][a[i] as usize] += 1;
    }
    let mut c = vec![vec![m+1; m as usize]; m as usize];
    for i in 0..m { for j in 0..m {
        for k in 1..=m {
            if (i * j * k + 1) % m == 0 {
                c[i as usize][j as usize] = k;
                break;
            }
        }
    }}

    for _ in 0..q {
        let (l, r) = (next::<usize>(), next::<usize>());
        if m == 1 { println!("1"); continue; }

        let mut ans = m+1;
        for a1 in 0..m as usize { for a2 in 0..m as usize {
            if a1 == a2 {
                if p[r][a1] - p[l-1][a1] > 1 { ans = ans.min(c[a1][a1]); }
            } else {
                if p[r][a1] - p[l-1][a1] > 0 && p[r][a2] - p[l-1][a2] > 0 { ans = ans.min(c[a1][a2]); }
            }
        }}
        println!("{}", if ans > m { -1 } else { ans });
    }
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