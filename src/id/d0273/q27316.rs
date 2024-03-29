// BOJ 27316 [Clock is Ticking Again]
const MAX: usize = 201_000;
const KMAX: usize = 555;
pub fn main() { read();
    let mut a = vec![0; MAX];
    for _ in 0..next() {
        a[next::<usize>()+1] = 1;
    }
    let mut p = vec![0; MAX];
    for i in 1..MAX {
        p[i] = p[i-1] + a[i];
    }

    let mut dp = vec![false; MAX];
    dp[1] = true;
    for i in 1..KMAX {
        let mut tp = vec![false; MAX];
        for j in 1..MAX {
            if !dp[j] { continue; }
            if p[j] - p[j-1] == 0 {
                if j+1 >= MAX { println!("YES"); return; }
                dp[j+1] = true;
            }
            let l = if j+i-1 < MAX { j+i-1 } else { MAX-1 };
            let r = if j+i*2-1 < MAX { j+i*2-1 } else { MAX-1 };
            if p[r] - p[l] == 0 && i+1 < KMAX {
                if r+1 >= MAX { println!("YES"); return; }
                tp[r+1] = true;
            }
        }
        dp = tp;
    }
    println!("NO");
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