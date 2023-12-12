pub fn main() { read();
    for _ in 0..next() {
        let n = next::<usize>();
        let (mut x, mut c, mut f) = (0, 0, false);
        let a = (0..n).map(|_| next::<u8>()).collect::<Vec<_>>();
        for &k in &a {
            x ^= k;
            if k == 3 { c += 1; }
            f |= k == 1 || k == 2;
        }
        if c == 0 { println!("0"); }
        else if x != 3 { println!("1"); }
        else if f {
            let mut g = false;
            for i in 0..n {
                if a[i] != 0 { g |= a[i] != 3; break; }
            }
            for i in (0..n).rev() {
                if a[i] != 0 { g |= a[i] != 3; break; }
            }
            if g { println!("1"); }
            else { println!("2"); }
        }
        else if n == 1 { println!("-1"); }
        else if n == c { println!("3"); }
        else { println!("2"); }
    }
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut BUF).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}