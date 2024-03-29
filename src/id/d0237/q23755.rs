// BOJ 23755 [Card Control (Hard)]
// Supported by GitHub Copilot

fn win(c: &[bool], n: usize) -> bool {
    let mut cnt = 0;
    for i in (0..n).step_by(2) {
        cnt += if c[i] && !c[i+1] { 1 } else if !c[i] && c[i+1] { -1 } else { 0 };
    }
    cnt > 0
}
pub fn main() { read();
    let n = next::<usize>();
    let mut c = vec![false; n<<1];
    for i in 0..n<<1 { c[i] = next::<char>() == 'O'; }

    if win(&c, n<<1) { println!("0"); return; }
    if n % 2 == 1 { println!("1"); return; }
    if c[0..n].iter().all(|&x| !x) { println!("3"); return; }

    let mut p = vec![0; n<<1];
    for i in (0..n<<1).step_by(2) {
        if c[i] && !c[i+1] { p[i] = 1; }
        else if !c[i] && c[i+1] { p[i] = -1; }
    }
    let mut s = vec![0; n<<1];
    s[(n<<1)-1] = p[(n<<1)-1];
    for i in (0..(n<<1)-1).rev() { s[i] = s[i+1] + p[i]; }
    s.push(0);

    p = vec![0; n<<1];
    for i in (2..n<<1).step_by(2) {
        if c[i-1] && !c[i] { p[i] = 1; }
        else if !c[i-1] && c[i] { p[i] = -1; }
    }
    let mut t = vec![0; n<<1];
    t[0] = p[0];
    for i in 1..n<<1 { t[i] = t[i-1] + p[i]; }
    t.push(0);

    for i in 1..n<<1 {
        if i % 2 == 1 {
            let c = s[i+1] + t[i-1] + if c[i] && !c[0] { 1 } else if !c[i] && c[0] { -1 } else { 0 };
            if c > 0 {
                println!("1"); return; }
        } else {
            let c = s[i+1] + t[i-1]
                + if c[i] && !c[0] { 1 } else if !c[i] && c[0] { -1 } else { 0 }
                + if c[i-1] && !c[i+1] { 1 } else if !c[i-1] && c[i+1] { -1 } else { 0 };
            if c > 0 {
                println!("1"); return; }
        }
    }
    println!("2");
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