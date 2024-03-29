// BOJ 6599 [The Tower of Babylon]
pub fn main() { read();
    for tc in 1.. {
        let n = next::<usize>();
        if n == 0 { break; }

        let mut b = vec![];
        for _ in 0..n {
            let (x, y, z) = (next::<i32>(), next::<i32>(), next::<i32>());
            b.push((x, y, z));
            b.push((x, z, y));
            b.push((y, z, x));
        }

        let mut adj = vec![vec![]; b.len()];
        let mut ind = vec![0; b.len()];
        for i in 0..b.len() {
            for j in 0..b.len() {
                if (b[i].0 < b[j].0 && b[i].1 < b[j].1) || (b[i].0 < b[j].1 && b[i].1 < b[j].0) {
                    adj[i].push(j);
                    ind[j] += 1;
                }
            }
        }

        let mut q = vec![];
        let mut dp = vec![0; b.len()];
        for i in 0..b.len() {
            if ind[i] == 0 {
                q.push(i);
                dp[i] = b[i].2;
            }
        }

        while !q.is_empty() {
            let x = q.pop().unwrap();
            for &y in &adj[x] {
                dp[y] = dp[y].max(dp[x] + b[y].2);
                ind[y] -= 1;
                if ind[y] == 0 { q.push(y); }
            }
        }

        println!("Case {}: maximum height = {}", tc, dp.iter().max().unwrap());
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut *std::ptr::addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}