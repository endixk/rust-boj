// BOJ 15074 [Removal Game]
const INF: u32 = 0x3f3f3f3f;
fn gcd(a: u32, b: u32) -> u32 { if b == 0 { a } else { gcd(b, a % b) } }
pub fn main() { read();
    let mut dp = [[0; 100]; 100];
    let mut cd = [[0; 1001]; 1001];
    for i in 1..1000 { for j in i..=1000 {
        let x = gcd(i as u32, j as u32);
        cd[i][j] = x; cd[j][i] = x;
    }}
    loop {
        let n = next::<usize>(); if n == 0 { break; }
        let a = (0..n).map(|_| next::<u32>()).collect::<Vec<_>>();
        dp[1].fill(0);
        for i in 3..n { dp[i].fill(INF); }
        for i in 0..n {
            dp[2][i] = cd[a[i] as usize][a[(i+2)%n] as usize];
        }
        for x in 3..n {
            for i in 0..n {
                for j in 1..x {
                    dp[x][i] = dp[x][i].min(dp[j][i] + dp[x-j][(i+j)%n])
                }
                dp[x][i] += cd[a[i] as usize][a[(i+x)%n] as usize];
            }
        }

        let mut ans = INF;
        for i in 0..n-1 { for j in i+1..n {
            let sz = j - i;
            ans = ans.min(dp[sz][i] + dp[n-sz][j] + cd[a[i] as usize][a[j] as usize]);
        }}
        println!("{}", ans);
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