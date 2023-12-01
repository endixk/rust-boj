// BOJ 26585 [Floor Cleaner]
// Supported by GitHub Copilot

pub fn main() { read();
    let mut map = vec![vec![false; 10]; 10];
    let (mut x, mut y) = (0, 0);
    for j in 0..10 {
        let s = next::<String>();
        for (i, c) in s.chars().enumerate() {
            map[i][j] = c == '-';
            if c == 'B' { x = i; y = j; }
        }
    }

    let locs = unsafe { IT.as_mut().unwrap().collect::<Vec<_>>() };
    let locs = locs.iter().map(|&s| (s[1..2].parse::<usize>().unwrap(), s[3..4].parse::<usize>().unwrap())).collect::<Vec<_>>();
    for &(i, j) in locs.iter() { map[i][j] = false; }

    let mut q = std::collections::VecDeque::new();
    q.push_back((x, y));
    const DX: [i32; 4] = [0, 0, 1, -1];
    const DY: [i32; 4] = [1, -1, 0, 0];
    while let Some((i, j)) = q.pop_front() {
        for k in 0..4 {
            let (ni, nj) = (i as i32 + DX[k], j as i32 + DY[k]);
            if ni < 0 || ni >= 10 || nj < 0 || nj >= 10 { continue; }
            let (ni, nj) = (ni as usize, nj as usize);
            if map[ni][nj] {
                map[ni][nj] = false;
                q.push_back((ni, nj));
            }
        }
    }

    let mut ans = locs.clone();
    for i in 0..10 {
        for j in 0..10 {
            if map[i][j] { ans.push((i, j)); }
        }
    }
    ans.sort_unstable_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));

    for (i, j) in ans { println!("({},{})", i, j); }
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