// BOJ 9520 [Traveller]
const INF: u32 = 0x3f3f3f3f;
fn go(dp: &mut Vec<Vec<u32>>, i: usize, j: usize, a: &Vec<Vec<u32>>) -> u32 {
    if dp[i][j] != 0 { return dp[i][j]; }
    if i == j { return if i == 0 { 0 } else { INF }; }

    let mut ret = INF;
    if i < j {
        if i+1 == j {
            for k in 0..=i {
                ret = ret.min(go(dp, i, k, a) + a[k][j]);
            }
        } else { ret = go(dp, i, j-1, a) + a[j-1][j]; }
    } else {
        if i == j+1 {
            for k in 0..=j {
                ret = ret.min(go(dp, k, j, a) + a[i][k]);
            }
        } else { ret = go(dp, i-1, j, a) + a[i][i-1]; }
    }
    dp[i][j] = ret;
    ret
}
pub fn main() { read();
    let n = next::<usize>();
    let mut a = vec![vec![0; n]; n];
    for i in 0..n { for j in 0..n { a[i][j] = next::<u32>(); } }

    let mut dp = vec![vec![0; n]; n];
    let mut ans = INF;
    for i in 0..n-1 {
        ans = ans.min(go(&mut dp, i, n-1, &a));
        ans = ans.min(go(&mut dp, n-1, i, &a));
    }
    println!("{}", ans);
}

#[allow(unused_macros)] macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_macros)] macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_imports)] use println;
#[allow(unused_imports)] use print;
#[allow(unused_imports)] use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
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