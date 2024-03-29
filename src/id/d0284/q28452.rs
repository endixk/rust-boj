// BOJ 28452 [Bullet Hell]
pub fn main() { read();
    let (n, m, t) = (next::<u32>(), next::<u32>(), next::<u32>());
    let (x, y) = (next::<u32>(), next::<u32>());
    let k = next::<u32>();
    let b = (0..k).map(|_| (next::<u32>(), next::<u32>())).collect::<Vec<_>>();

    for i in 0..n { for j in 0..m {
        let d = (if i < x { x - i } else { i - x }).max(if j < y { y - j } else { j - y });
        let mut k = 111;
        for &(x, y) in &b {
            let d = (if i < x { x - i } else { i - x }).max(if j < y { y - j } else { j - y });
            if d < k { k = d; }
        }
        if d < k && t < k { println!("YES"); return; }
    }}
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