// BOJ 14450 [Hoof, Paper, Scissors (Gold)]
pub fn main() { read();
    let (n, k) = (next::<usize>(), next::<usize>());
    let mut dp = [0; 84];
    let mut tp = [0; 84];
    let mut flag = true;
    let c = match next::<char>() { 'H' => 0, 'P' => 1, _ => 2 };
    dp[c] = 1;
    for _ in 1..n {
        let (dp, tp) = if flag { (&mut dp, &mut tp) } else { (&mut tp, &mut dp) };
        let c = match next::<char>() { 'H' => 0, 'P' => 1, _ => 2 };
        for j in 0..3 {
            let x = if j == c { 1 } else { 0 };
            for i in 1..=k {
                tp[i<<2|j] = dp[i<<2|j].max(dp[i-1<<2|(j+1)%3]).max(dp[i-1<<2|(j+2)%3]) + x;
            }
            tp[j] = dp[j] + x;
        }
        flag = !flag;
    }

    let dp = if flag { &dp } else { &tp };
    println!("{}", dp.iter().max().unwrap());
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