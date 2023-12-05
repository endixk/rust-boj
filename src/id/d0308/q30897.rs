// BOJ 30897 [Down to Zero]
// Supported by GitHub Copilot

pub fn main() { read();
    let n = next::<usize>();
    let a = (0..n).map(|_| next::<usize>()).collect::<Vec<_>>();

    let mut p = vec![n; 20001];
    let mut op = vec![' '; n-1];
    let mut x = a[0] + 10000;
    p[x] = 0;
    for i in 1..n {
        if x + a[i] > 20000 {
            op[i-1] = '-';
            x -= a[i];
        } else {
            op[i-1] = '+';
            x += a[i];
        }
        if p[x] == n {
            p[x] = i; continue;
        }
        println!("YES");
        for j in 0..p[x] {
            print!("{}*", a[j]);
        }
        print!("{}*({}", a[p[x]], a[p[x]+1]);
        for j in p[x]+2..=i {
            let q = if op[p[x]] == '+' {
                if op[j-1] == '+' { '+' } else { '-' }
            } else {
                if op[j-1] == '+' { '-' } else { '+' }
            };
            print!("{}{}", q, a[j]);
        }
        print!(")");
        for j in i+1..n {
            print!("*{}", a[j]);
        }
        println!();
        return;
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