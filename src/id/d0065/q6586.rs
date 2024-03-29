// BOJ 6586 [Error Correction]
pub fn main() { read();
    loop {
        let n = next::<usize>();
        if n == 0 { break; }

        let mut r = vec![0; n];
        let mut c = vec![0; n];
        for i in 0..n { for j in 0..n {
            match next::<u8>() {
                1 => { r[i] += 1; c[j] += 1; },
                _ => {}
            }
        }}

        let p = r.iter().filter(|&&x| x % 2 == 1).count();
        let q = c.iter().filter(|&&x| x % 2 == 1).count();
        if p == 0 && q == 0 { println!("OK"); }
        else if p == 1 && q == 1 {
            println!("Change bit ({},{})", r.iter().position(|&x| x % 2 == 1).unwrap() + 1, c.iter().position(|&x| x % 2 == 1).unwrap() + 1);
        } else { println!("Corrupt"); }
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut *std::ptr::addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}