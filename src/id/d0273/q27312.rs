// BOJ 27312 [Setting Properties is Difficult]
pub fn main() {
    readline();
    let (m, n, _) = (next::<usize>(), next::<usize>(), next::<usize>());
    readline();
    let a = (0..n).map(|_| next::<usize>()).collect::<Vec<_>>();
    let mut p = vec![1; n];
    for i in 0..m {
        println!("? {} {}", i+1, i+1);
        readline();
        let x = next::<usize>();
        p[i] = if x == a[i] { x-1 } else { x+1 };
    }
    println!("{}", "! ".to_string() + &p.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
}

macro_rules! println { ($($t:tt)*) => {
    SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap());
    SO.with(|c| c.borrow_mut().flush().unwrap());
};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug};
static mut BUF: String = String::new();
static mut IT: Option<SplitAsciiWhitespace> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn readline() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_line(&mut *std::ptr::addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}