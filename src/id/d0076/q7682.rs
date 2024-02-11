// BOJ 7682 [Tic-Tac-Toe]
fn done(b: [u8; 9]) -> bool {
    if b.iter().all(|&x| x != 0) { return true; }
    if  (b[0] == 1 && b[1] == 1 && b[2] == 1) || (b[0] == 2 && b[1] == 2 && b[2] == 2) ||
        (b[3] == 1 && b[4] == 1 && b[5] == 1) || (b[3] == 2 && b[4] == 2 && b[5] == 2) ||
        (b[6] == 1 && b[7] == 1 && b[8] == 1) || (b[6] == 2 && b[7] == 2 && b[8] == 2) ||
        (b[0] == 1 && b[3] == 1 && b[6] == 1) || (b[0] == 2 && b[3] == 2 && b[6] == 2) ||
        (b[1] == 1 && b[4] == 1 && b[7] == 1) || (b[1] == 2 && b[4] == 2 && b[7] == 2) ||
        (b[2] == 1 && b[5] == 1 && b[8] == 1) || (b[2] == 2 && b[5] == 2 && b[8] == 2) ||
        (b[0] == 1 && b[4] == 1 && b[8] == 1) || (b[0] == 2 && b[4] == 2 && b[8] == 2) ||
        (b[2] == 1 && b[4] == 1 && b[6] == 1) || (b[2] == 2 && b[4] == 2 && b[6] == 2) { return true; }
    false
}
fn hash(b: [u8; 9]) -> u32 {
    let mut h = 0;
    for i in 0..9 { h = h * 3 + b[i] as u32; }
    h
}
use std::collections::HashSet;
fn go(set: &mut HashSet<u32>, b: &mut [u8; 9], turn: u8) {
    if done(*b) { set.insert(hash(*b)); return; }
    for i in 0..9 {
        if b[i] == 0 {
            b[i] = turn;
            go(set, b, 3 - turn);
            b[i] = 0;
        }
    }
}
pub fn main() { read();
    let mut b = [0; 9];
    let mut set = HashSet::new();
    go(&mut set, &mut b, 1);

    loop {
        let s = next::<String>();
        if s == "end" { break; }
        let mut b = [0; 9];
        for (i, c) in s.chars().enumerate() { b[i] = if c == 'X' { 1 } else if c == 'O' { 2 } else { 0 }; }
        println!("{}", if set.contains(&hash(b)) { "valid" } else { "invalid" });
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