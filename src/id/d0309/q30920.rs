// BOJ 30920 [Gift Sequence]
// Supported by GitHub Copilot

fn cycle(a: &[usize], v: &mut [bool], i: usize) -> Vec<usize> {
    let mut ret = vec![i];
    v[i] = true;
    let mut j = a[i];
    while j != i {
        v[j] = true;
        ret.push(j);
        j = a[j];
    }
    ret
}
pub fn main() { read();
    let n = next::<usize>();
    let a = (0..n).map(|_| next::<usize>() - 1).collect::<Vec<_>>();

    let mut ans = vec![];
    let mut v = vec![false; n];
    for i in 0..n {
        if v[i] { continue; }
        let c = cycle(&a, &mut v, i);
        if c.len() == 1 { continue; }
        for x in c {
            ans.push(x + 1);
        }
        ans.push(i + 1);
    }

    println!("{}", ans.len());
    for x in ans {
        println!("{} {} {}", x, 1, n);
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