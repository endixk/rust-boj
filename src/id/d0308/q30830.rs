// BOJ 30830 [Two Chessboards]
// Supported by GitHub Copilot

pub fn main() { read();
    let n = next::<usize>();
    let mut adj = vec![vec![]; n<<1];
    let mut col: Vec<Option<usize>> = vec![None; n];
    let mut row: Vec<Option<usize>> = vec![None; n];
    for i in 0..n<<1 {
        let (c, r) = (next::<usize>()-1, next::<usize>()-1);
        match col[c] {
            None => col[c] = Some(i),
            Some(j) if j == n<<1 => { println!("-1"); return; },
            Some(j) => { adj[j].push(i); adj[i].push(j); col[c] = Some(n<<1); }
        }
        match row[r] {
            None => row[r] = Some(i),
            Some(j) if j == n<<1 => { println!("-1"); return; },
            Some(j) => { adj[j].push(i); adj[i].push(j); row[r] = Some(n<<1); }
        }
    }

    let mut ans = 0;
    let mut vis = vec![false; n<<1];
    for i in 0..n<<1 {
        if vis[i] { continue; }
        let (mut j, mut c, mut l, mut f) = (i, 0, 0, true);
        while !vis[j] {
            l += 1;
            if (f && j < n) || (!f && j >= n) { c += 1; }
            vis[j] = true;
            if vis[adj[j][0]] { j = adj[j][1]; }
            else { j = adj[j][0]; }
            f = !f;
        }
        ans += c.min(l-c);
    }
    println!("{}", ans/2);
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