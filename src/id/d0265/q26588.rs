// BOJ 26588 [Pegs]
// Supported by GitHub Copilot

// 0: up, 1: down, 2: left, 3: right
fn scan(b: &Vec<Vec<char>>, n: usize, m: usize) -> Vec<(usize, usize, u8)> {
    let mut ret = vec![];
    for i in 0..n { for j in 0..m {
        if b[i][j] != '@' { continue; }
        if i > 1 && b[i-1][j] == '@' && b[i-2][j] == '.' { ret.push((i, j, 0)); }
        if i < n-2 && b[i+1][j] == '@' && b[i+2][j] == '.' { ret.push((i, j, 1)); }
        if j > 1 && b[i][j-1] == '@' && b[i][j-2] == '.' { ret.push((i, j, 2)); }
        if j < m-2 && b[i][j+1] == '@' && b[i][j+2] == '.' { ret.push((i, j, 3)); }
    }}
    ret
}
fn go(b: &mut Vec<Vec<char>>, n: usize, m: usize, c: usize) -> bool {
    if c == 1 { return true; }
    let v = scan(b, n, m);
    let mut ret = false;
    for (i, j, d) in v {
        b[i][j] = '.';
        if d == 0 {
            b[i-1][j] = '.'; b[i-2][j] = '@';
            ret |= go(b, n, m, c-1);
            b[i-1][j] = '@'; b[i-2][j] = '.';
        } else if d == 1 {
            b[i+1][j] = '.'; b[i+2][j] = '@';
            ret |= go(b, n, m, c-1);
            b[i+1][j] = '@'; b[i+2][j] = '.';
        } else if d == 2 {
            b[i][j-1] = '.'; b[i][j-2] = '@';
            ret |= go(b, n, m, c-1);
            b[i][j-1] = '@'; b[i][j-2] = '.';
        } else if d == 3 {
            b[i][j+1] = '.'; b[i][j+2] = '@';
            ret |= go(b, n, m, c-1);
            b[i][j+1] = '@'; b[i][j+2] = '.';
        }
        b[i][j] = '@';
    }
    ret
}
pub fn main() { read();
    for _ in 0..next() {
        let s = next::<String>();
        let w = s.split_whitespace().collect::<Vec<_>>();
        let (n, m) = (w[0].parse().unwrap(), w[1].parse().unwrap());
        let mut b = vec![vec![' '; m]; n];
        let mut cnt = 0;
        for i in 0..n {
            let s = next::<String>();
            for (j, c) in s.chars().enumerate() {
                b[i][j] = c;
                if c == '@' { cnt += 1; }
            }
        }
        println!("{}", if go(&mut b, n, m, cnt) { "Solvable!" } else { "Impossible." });
    }
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug};
static mut BUF: String = String::new();
static mut IT: Option<std::str::Split<'static, &'static str>> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut *std::ptr::addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split("\n"));
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}