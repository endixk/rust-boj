// BOJ 18185 [Ramyeon Buying (Small)]
pub fn main() { read();
    let n = next::<usize>();
    let mut a = vec![0; 10002];
    for i in 0..n { a[i] = next::<i32>(); }

    let mut ans = 0;
    for i in 0..n {
        let mut f = 0;
        if a[i] < a[i+1] { f = a[i+1] - a[i]; a[i+2] -= f; }
        let k = a[i].min(a[i+1]).min(a[i+2]);
        if k > 0 { ans += k * 7; a[i] -= k; a[i+1] -= k; a[i+2] -= k; }
        let k = a[i].min(a[i+1]);
        if k > 0 { ans += k * 5; a[i] -= k; a[i+1] -= k; }
        ans += a[i] * 3;
        a[i+2] += f;
    }
    println!("{}", ans);
}

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