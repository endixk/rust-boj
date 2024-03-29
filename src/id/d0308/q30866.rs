// BOJ 30866 [Not a SAT problem]
// Supported by GitHub Copilot

pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());
    let mut ans = [0; 500000];
    't: for i in 1..=m {
        let k = next::<i32>();
        for j in 0..k {
            let x = next::<i32>();
            if x > 0 {
                let x = x as usize - 1;
                if ans[x] == i<<1|1 {
                    for _ in j+1..k { unsafe { IT.as_mut().unwrap().next(); } }
                    continue 't;
                }
                ans[x] = i<<1;
            } else {
                let x = -x as usize - 1;
                if ans[x] == i<<1 {
                    for _ in j+1..k { unsafe { IT.as_mut().unwrap().next(); } }
                    continue 't;
                }
                ans[x] = i<<1|1;
            }
        }
        println!("YES");
        for i in 0..n { print!("{} ", ans[i] & 1); }
        println!();
        return;
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