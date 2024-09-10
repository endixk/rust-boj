// BOJ 17613 [Jump]
use std::collections::HashMap;
fn solve(i: u32, j: u32, dp: &mut HashMap<u32, u32>) -> u32 {
    if i > j { return 0; }
    if i == 1 && dp.contains_key(&j) { return dp[&j]; }
    if j == 1 { return 1; }
    let k = ((j+2).next_power_of_two()-1)>>1;
    let ret = if k<<1 == j {
        solve(i, j-1, dp).max(k.count_ones() << 1)
    } else if k == j {
        solve(i, j-1, dp).max(k.count_ones())
    } else if i > k {
        k.count_ones() + solve(i-k, j-k, dp)
    } else {
        k.count_ones().max(solve(i, k-1, dp)).max(k.count_ones() + solve(1, j-k, dp))
    };
    if i == 1 { dp.insert(j, ret); }
    ret
}
pub fn main() { read();
    let mut dp = HashMap::new();
    for _ in 0..next() {
        let x= solve(next(), next(), &mut dp);
        println!("{}", x);
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