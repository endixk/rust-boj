fn score(a: &Vec<Vec<bool>>, bit: usize, ans: &Vec<bool>, n: usize, m: usize) -> i32 {
    let mut cnt = 0;
    for i in 0..m {
        let (mut z, mut o) = (0, 0);
        for j in 0..n {
            if 1 << j & bit == 0 { continue; }
            if a[j][i] { o += 1; }
            else { z += 1; }
        }
        let k = if o > z { true } else { false };
        if k == ans[i] { cnt += 1; }
    }
    cnt
}
pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());
    let mut ans = vec![false; m];
    for i in 0..m { ans[i] = next::<usize>() == 1; }

    let mut a = vec![vec![false; m]; n];
    let mut max = 0;
    for i in 0..n {
        let mut cnt = 0;
        for j in 0..m {
            a[i][j] = next::<usize>() == 1;
            if a[i][j] == ans[j] { cnt += 1; }
        }
        max = max.max(cnt);
    }

    for bit in 0..1<<n {
        let c = (0..n).filter(|&i| 1 << i & bit != 0).count();
        if c % 2 == 0 { continue; }
        if score(&a, bit, &ans, n, m) > max { println!("1"); return; }
    }
    println!("0");
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