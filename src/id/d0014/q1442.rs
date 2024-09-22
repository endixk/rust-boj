// BOJ 1442 [Cool Numbers]
fn go(dp: &mut Vec<Vec<Vec<u32>>>, i: usize, j: usize, k: usize) -> u32 {
    if k == 0 { return 1; }
    if dp[i][j][k] != 0 { return dp[i][j][k]; }
    let ret = if i == 1 && j == 1 {
        go(dp, 1, 0, k-1)
    } else if i == 0 && j == 0 {
        go(dp, 0, 1, k-1)
    } else {
        go(dp, j, if j == 2 { 2 } else { 0 }, k-1) + go(dp, j, 1, k-1)
    };
    dp[i][j][k] = ret;
    ret
}
fn cool(dp: &mut Vec<Vec<Vec<u32>>>, i: usize, j: usize, k: usize, x: u32) -> u32 {
    let cap = (1 << k) - 1;
    if x >= cap { return go(dp, i, j, k); }

    let mut ret = 0;
    if i != 0 || j != 0 { ret += cool(dp, j, if j == 2 { 2 } else { 0 }, k-1, x); }
    if (i != 1 || j != 1) && x >= cap + 1 >> 1 { ret += cool(dp, j, 1, k-1, x & !(cap + 1 >> 1)); }
    ret
}
pub fn main() { read();
    let (l, r) = (next::<u32>(), next::<u32>());
    let (lk, rk) = (l.next_power_of_two().trailing_zeros() as usize, (r+1).next_power_of_two().trailing_zeros() as usize);
    let mut dp = vec![vec![vec![0; 32]; 3]; 3];
    if l == 0 {
        println!("{}", r + 1 - cool(&mut dp, 2, 2, rk, r));
    } else {
        let rc = r + 1 - cool(&mut dp, 2, 2, rk, r);
        let lc = l - cool(&mut dp, 2, 2, lk, l - 1);
        println!("{}", rc - lc);
    }
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