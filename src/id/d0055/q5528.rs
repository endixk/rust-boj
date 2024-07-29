// BOJ 5528 [Take The IOI Train]
fn go(dp: &mut Vec<i32>, i: usize, j: usize, f: usize,
      a: &Vec<bool>, b: &Vec<bool>, n: usize, m: usize) -> i32 {
    if i == n && j == m { return 0; }
    if dp[i<<12|j<<1|f] != -1 { return dp[i<<12|j<<1|f]; }

    let mut ret = 0;
    if i < n {
        if f == 1 && a[i] { ret = ret.max(go(dp, i+1, j, 0, a, b, n, m) + 1); }
        else if f == 0 && !a[i] { ret = ret.max(go(dp, i+1, j, 1, a, b, n, m) + 1); }
    }
    if j < m {
        if f == 1 && b[j] { ret = ret.max(go(dp, i, j+1, 0, a, b, n, m) + 1); }
        else if f == 0 && !b[j] { ret = ret.max(go(dp, i, j+1, 1, a, b, n, m) + 1); }
    }
    dp[i<<12|j<<1|f] = ret;
    ret
}
pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());
    let a = next::<String>().chars().map(|c| c == 'I').collect::<Vec<_>>();
    let b = next::<String>().chars().map(|c| c == 'I').collect::<Vec<_>>();

    let mut dp = vec![-1; 1<<23];
    let mut ans = 0;
    for i in 0..=n { for j in 0..=m {
        ans = ans.max(go(&mut dp, i, j, 1, &a, &b, n, m));
    }}
    println!("{}", if ans == 0 { 0 } else if ans & 1 == 0 { ans - 1 } else { ans });
}

#[allow(unused_macros)] macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_macros)] macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_imports)] use println;
#[allow(unused_imports)] use print;
use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
static mut BUF: String = String::new();
static mut IT: Option<SplitAsciiWhitespace> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut *addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}