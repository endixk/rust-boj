// BOJ 23740 [Reorganizing Bus Schedules]
// Supported by GitHub Copilot

pub fn main() { read();
    let n = next::<usize>();
    let mut v = vec![];
    for _ in 0..n {
        let (s, e, c) = (next::<usize>(), next::<usize>(), next::<usize>());
        v.push((s, false, e, c));
        v.push((e, true,  e, c));
    }
    v.sort_unstable();

    let (mut cs, mut ce, mut cc) = (0, 0, 0);
    let (mut cnt, mut ans) = (0, vec![]);
    for (x, t, e, c) in v {
        if cc == 0 && t { continue; }
        if cc == 0 { cs = x; ce = e; cc = c; continue; }
        if !t {
            if cc > c { cc = c; }
            if ce < e { ce = e; }
        } else {
            if ce == e {
                cnt += 1;
                ans.push((cs, ce, cc));
                cc = 0;
            }
        }
    }

    println!("{}", cnt);
    for (s, e, c) in ans {
        println!("{} {} {}", s, e, c);
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
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::with_capacity(1<<18, stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut BUF).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}