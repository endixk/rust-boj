// BOJ 23749 [Card Control]
// Supported by GitHub Copilot

fn win(c: &[bool], n: usize) -> bool {
    let mut cnt = 0;
    for i in (0..n).step_by(2) {
        cnt += if c[i] && !c[i+1] { 1 } else if !c[i] && c[i+1] { -1 } else { 0 };
    }
    cnt > 0
}
pub fn main() { read();
    let n = next::<usize>() * 2;
    let c = (0..n).map(|_| next::<char>() == 'O').collect::<Vec<_>>();
    let mut vis = std::collections::HashSet::new();
    let mut q = std::collections::VecDeque::new();
    q.push_back(c.clone()); vis.insert(c);
    let mut ans = 0;
    loop {
        let sz = q.len();
        for _ in 0..sz {
            let c = q.pop_front().unwrap();
            if win(&c, n) {
                println!("{}", ans);
                return;
            }
            for i in 0..n {
                let mut d = vec![c[i]];
                for j in 0..n {
                    if i == j { continue; }
                    d.push(c[j]);
                }
                if vis.insert(d.clone()) { q.push_back(d); }
            }
        }
        ans += 1;
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