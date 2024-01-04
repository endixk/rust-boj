// BOJ 30519 [A Put-up Job]
// Supported by GitHub Copilot

// 0: RR, 1: RP, 2: RS, 3: PR, 4: PP, 5: PS, 6: SR, 7: SP, 8: SS
#[inline] fn get(a: char, b: char) -> usize {
    match (a, b) {
        ('R', 'R') => 0, ('R', 'P') => 1, ('R', 'S') => 2,
        ('P', 'R') => 3, ('P', 'P') => 4, ('P', 'S') => 5,
        ('S', 'R') => 6, ('S', 'P') => 7, ('S', 'S') => 8,
        _ => unreachable!(),
    }
}
pub fn main() { read();
    let c = next::<char>();
    let s = next::<String>();
    let s = s.chars().collect::<Vec<_>>();

    const MOD: usize = 1_000_000_007;
    let mut dp = vec![0; 9];
    for i in 0..s.len() {
        let mut tp = dp.clone();
        tp[get('R', s[i])] = (tp[get('R', s[i])] + dp[get('R', 'R')] + if s[i] != 'R' { dp[get('P', 'R')] } else { 0 } + dp[get('S', 'R')] + if c == 'R' { 1 } else { 0 }) % MOD;
        tp[get('P', s[i])] = (tp[get('P', s[i])] + dp[get('R', 'P')] + dp[get('P', 'P')] + if s[i] != 'P' { dp[get('S', 'P')] } else { 0 } + if c == 'P' { 1 } else { 0 }) % MOD;
        tp[get('S', s[i])] = (tp[get('S', s[i])] + if s[i] != 'S' { dp[get('R', 'S')] } else { 0 } + dp[get('P', 'S')] + dp[get('S', 'S')] + if c == 'S' { 1 } else { 0 }) % MOD;
        dp = tp;
    }

    println!("{}", dp.iter().sum::<usize>() % MOD);
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