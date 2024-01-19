// BOJ 28068 [I Am Knowledge]
pub fn main() { read();
    let n = next::<usize>();
    let (mut v, mut w) = (vec![], vec![]);
    for _ in 0..n {
        let (a, b) = (next::<i64>(), next::<i64>());
        if a <= b { v.push((a, b)); }
        else { w.push((a, b)); }
    }

    v.sort_unstable_by(|x, y| x.0.cmp(&y.0).then(y.1.cmp(&x.1)));
    w.sort_unstable_by(|x, y| y.1.cmp(&x.1).then(x.0.cmp(&y.0)));
    let mut fun = 0;
    for (a, b) in v {
        if fun < a { println!("0"); return; }
        fun += b - a;
    }
    for (a, b) in w {
        if fun < a { println!("0"); return; }
        fun += b - a;
    }
    println!("1");
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