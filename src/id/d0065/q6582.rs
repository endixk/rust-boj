// BOJ 6582 [Artificial Intelligence?]
pub fn main() { read();
    let mut wva: [Option<f64>; 3] = [None, None, None];
    let (mut q, mut c) = (1, 0);
    while peek() {
        let word = next::<String>();
        let wch = word.clone().chars().collect::<Vec<_>>();
        if !word.contains("=") { continue; }
        let r = match word.chars().next().unwrap() {
            'P' => wch.iter().rposition(|&c| c == 'W').unwrap(),
            'U' => wch.iter().rposition(|&c| c == 'V').unwrap(),
            'I' => wch.iter().rposition(|&c| c == 'A').unwrap(),
            _ => unreachable!()
        };

        let s = word.chars().position(|c| c == '=').unwrap() + 1;
        let (e, f) = match word[..r].chars().rev().next().unwrap() {
            'm' => (r-1, 1e-3), 'k' => (r-1, 1e3), 'M' => (r-1, 1e6), _ => (r, 1e0)
        };
        let i = match word.chars().next().unwrap() {
            'P' => 0, 'U' => 1, 'I' => 2, _ => unreachable!()
        };
        wva[i] = Some(word[s..e].parse::<f64>().unwrap() * f);
        c += 1;

        if c == 2 {
            println!("Problem #{}", q);
            if let None = wva[0] {
                println!("P={:.2}W", wva[1].unwrap() * wva[2].unwrap());
            } else if let None = wva[1] {
                println!("U={:.2}V", wva[0].unwrap() / wva[2].unwrap());
            } else {
                println!("I={:.2}A", wva[0].unwrap() / wva[1].unwrap());
            }
            println!();
            wva = [None, None, None];
            c = 0;
            q += 1;
        }
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
