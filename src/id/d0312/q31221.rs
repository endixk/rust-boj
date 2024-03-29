// BOJ 31221 [Hard Number Guessing Game]
pub fn main() {
    let mut s = next(); s.pop();
    'x: for _ in 0..s.parse::<usize>().unwrap() {
        let (mut l, mut r) = (0u64, 10u64.pow(18));
        let mut c = 0;
        while l + 10 < r {
            c += 1;
            if c > 75 { panic!() }
            let q = (((r - l) as f64 / 2.0).sqrt() - 1e-9).ceil() as u64;
            println!("? {} {}", l, q);
            flush();
            match next().chars().next().unwrap() {
                '+' => (l, r) = (q * q + l + 1, r),
                '-' => (l, r) = (l, q * q + l - 1),
                '!' => return,
                _ => {
                    println!("! {}", q * q + l);
                    continue 'x;
                }
            }
        }
        for i in l.max(2) - 2..r + 2 {
            c += 1;
            if c > 75 { panic!() }
            println!("? {} {}", i, 0);
            flush();
            match next().chars().next().unwrap() {
                '0' => {
                    println!("! {}", i);
                    continue 'x;
                }
                '!' => return,
                _ => { continue; }
            }
        }
    }
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
use println;
use std::{io::*, cell::*, str::*, fmt::Debug};
static mut BUF: String = String::new();
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn next() -> String { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_line(&mut *std::ptr::addr_of_mut!(BUF)).unwrap());
    BUF.clone()
}}
fn flush() { SO.with(|c| c.borrow_mut().flush().unwrap()); }