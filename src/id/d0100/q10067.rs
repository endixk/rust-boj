// BOJ 10067 [Split the Sequence]
#[derive(Copy, Clone)]
struct Segment { a: i64, b: i64, i: u32 }
pub fn main() { read();
    let (n, k) = (next::<usize>(), next::<usize>());
    let a = (0..n).map(|_| next::<i64>()).collect::<Vec<_>>();
    let mut p = vec![0; n+1];
    for i in 0..n { p[i+1] = p[i] + a[i]; }

    let mut dp = vec![0; n+1];
    let mut tr = vec![vec![0u32; n+1]; k+1];
    for t in 1..=k {
        let mut tp = vec![-0x3f3f3f3f; n+1];
        let mut cht = std::collections::VecDeque::new();
        cht.push_back(Segment { a: 0, b: 0, i: 0 });
        for i in t..=n {
            while cht.len() > 1 {
                if cht[0].a * p[i] + cht[0].b > cht[1].a * p[i] + cht[1].b { break; }
                cht.pop_front();
            }
            tp[i] = cht[0].a * p[i] + cht[0].b;
            tr[t][i] = cht[0].i;

            let ns = Segment { a: p[i], b: dp[i] - p[i] * p[i], i: i as u32 };
            while cht.len() > 1 {
                let (a, b, c) = (cht[cht.len()-2], cht[cht.len()-1], ns);
                if (b.b - a.b) * (a.a - c.a) < (c.b - a.b) * (a.a - b.a) { break; }
                cht.pop_back();
            }
            cht.push_back(ns);
        }
        dp = tp;
    }

    println!("{}", dp[n]);
    let mut ans = vec![];
    let mut i = n;
    for t in (1..=k).rev() {
        i = tr[t][i] as usize;
        ans.push(i);
    }
    ans.iter().rev().for_each(|&x| print!("{} ", x));
    println!();
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut *std::ptr::addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}