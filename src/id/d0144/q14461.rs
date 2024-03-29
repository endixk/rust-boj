// BOJ 14461 [Why Did the Cow Cross the Road (Gold)]
const INF: i32 = 0x3f3f3f3f;
const OPS: usize = 300;
pub fn main() { read();
    let (n, t) = (next::<usize>(), next::<i32>());
    let mut b = [0; 1<<14];
    for i in 0..n { for j in 0..n { b[i<<7|j] = next::<i32>(); }}

    let mut ans = INF;
    let mut dp = [INF; 1<<14];
    dp[0] = 0;
    for k in 0..OPS {
        let mut tp = [INF; 1<<14];
        for i in 0..n { for j in 0..n {
            if dp[i<<7|j] == INF { continue; }
            if i > 0   { tp[i-1<<7|j] = tp[i-1<<7|j].min(dp[i<<7|j] + t + if k % 3 == 2 { b[i-1<<7|j] } else { 0 }); }
            if i < n-1 { tp[i+1<<7|j] = tp[i+1<<7|j].min(dp[i<<7|j] + t + if k % 3 == 2 { b[i+1<<7|j] } else { 0 }); }
            if j > 0   { tp[i<<7|j-1] = tp[i<<7|j-1].min(dp[i<<7|j] + t + if k % 3 == 2 { b[i<<7|j-1] } else { 0 }); }
            if j < n-1 { tp[i<<7|j+1] = tp[i<<7|j+1].min(dp[i<<7|j] + t + if k % 3 == 2 { b[i<<7|j+1] } else { 0 }); }
        }}
        dp = tp;
        ans = ans.min(dp[n-1<<7|n-1]);
    }
    println!("{}", ans);
}

use std::{io::*, cell::*, str::*, fmt::Debug};
static mut BUF: String = String::new();
static mut IT: Option<SplitAsciiWhitespace> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut *std::ptr::addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}