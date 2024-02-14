// BOJ 6604 [Matrix Chain Multiplication]
#[derive(Clone, Copy, Default)] struct Matrix { b: bool, r: u32, c: u32 }
pub fn main() { read();
    let mut v = [Matrix::default(); 26];
    for _ in 0..next() {
        let (i, r, c) = (next::<char>() as usize - b'A' as usize, next::<u32>(), next::<u32>());
        v[i] = Matrix { b: false, r, c };
    }

    'x: while peek() {
        let mut stk = vec![];
        let exp = next::<String>();
        let mut ans = 0;
        for c in exp.chars() {
            match c {
                '(' => stk.push(Matrix { b: true, r: 0, c: 0 }),
                ')' => {
                    while stk.len() > 1 {
                        let a = stk.pop().unwrap();
                        let b = stk.pop().unwrap();
                        if b.b { stk.push(a); break; }
                        if b.c != a.r { println!("error"); continue 'x; }
                        ans += b.r * b.c * a.c;
                        stk.push(Matrix { b: false, r: b.r, c: a.c });
                    }
                }
                _ => stk.push(v[c as usize - 'A' as usize]),
            }
        }
        println!("{ans}");
    }
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug, iter::Peekable};
static mut BUF: String = String::new();
static mut IT: Option<Peekable<SplitAsciiWhitespace>> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut BUF).unwrap());
    IT = Some(BUF.split_ascii_whitespace().peekable());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}
fn peek() -> bool { unsafe { IT.as_mut().unwrap().peek().is_some() } }
