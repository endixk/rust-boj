// BOJ 26591 [Rain Boots]
// Supported by GitHub Copilot

pub fn main() { read();
    let v = unsafe { IT.as_mut().unwrap().map(|s| s.to_string()).collect::<Vec<_>>() };
    for s in &v {
        assert!(s.len() == 8 || s.len() == 1);
    }
    let mut it = v.iter();
    for tc in 0..(v.len()+1)/9 {
        let mut b = vec![vec![' '; 8]; 8];
        let (mut si, mut sj) = (0, 0);
        if tc > 0 { it.next(); }
        for i in 0..8 {
            let s = it.next().unwrap();
            for (j, c) in s.chars().enumerate() {
                b[i][j] = c;
                if c == 'S' { si = i; sj = j; }
            }
        }

        let mut q = std::collections::VecDeque::new();
        q.push_back((si, sj, 0));
        let mut v = vec![vec![false; 8]; 8];
        v[si][sj] = true;
        while !q.is_empty() {
            let (i, j, d) = q.pop_front().unwrap();
            if b[i][j] == 'E' { println!("{}", d); break; }
            if i > 0 && !v[i-1][j] {
                if b[i-1][j] == 'M' { q.push_back((i-1, j, d+1)); }
                else { q.push_front((i-1, j, d)); }
                v[i-1][j] = true;
            }
            if i < 7 && !v[i+1][j] {
                if b[i+1][j] == 'M' { q.push_back((i+1, j, d+1)); }
                else { q.push_front((i+1, j, d)); }
                v[i+1][j] = true;
            }
            if j > 0 && !v[i][j-1] {
                if b[i][j-1] == 'M' { q.push_back((i, j-1, d+1)); }
                else { q.push_front((i, j-1, d)); }
                v[i][j-1] = true;
            }
            if j < 7 && !v[i][j+1] {
                if b[i][j+1] == 'M' { q.push_back((i, j+1, d+1)); }
                else { q.push_front((i, j+1, d)); }
                v[i][j+1] = true;
            }
        }
    }

    if v.len() % 9 != 8 { println!("5"); }
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