// BOJ 30787 [Tree within a tree]
// Supported by GitHub Copilot

pub fn main() {
    let s = read();
    let mut it = s.split_ascii_whitespace();

    const MOD: usize = 1_000_000_007;
    let (n, k) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let (mut ans, mut c) = (1, 2);
    for i in 2..=n {
        if i + k <= n + 1 {
            if i > k { ans = (ans + c * 3) % MOD; }
            else { ans = (ans + c) % MOD; }
        }
        c = c * 2 % MOD;
    }
    println!("{}", ans);
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug};
fn read() -> String {
    let mut s = String::new();
    SI.with(|c| c.borrow_mut().read_to_string(&mut s).unwrap());
    s
}
fn next<T: FromStr>(it: &mut SplitAsciiWhitespace) -> T where <T as FromStr>::Err: Debug {
    it.next().unwrap().parse().unwrap()
}