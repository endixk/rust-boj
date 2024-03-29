pub fn main() { read();
    let n = next::<usize>();
    let a = next::<String>().into_bytes().into_iter().map(|x| x - b'0').collect::<Vec<_>>();

    let mut ans = vec![0; n];
    let mut cnt = vec![0; 10];
    for i in 0..n>>1 {
        if a[i] == a[n-i-1] {
            cnt[a[i] as usize] += 2;
        } else {
            let k = if a[i] < a[n-i-1] { a[i] } else { a[n-i-1] };
            cnt[k as usize] += 1;
        }
    }
    if n & 1 == 1 { cnt[a[n>>1] as usize] += 1; }

    for i in 0..n>>1 {
        if a[i] != a[n-i-1] {
            let k = if a[i] > a[n-i-1] { a[i] } else { a[n-i-1] };
            cnt[k as usize] += 1;
        }
        for c in (1..=9).rev() {
            let req = if a[i] < c { 1 } else { 0 } + if a[n-i-1] < c { 1 } else { 0 };
            if cnt[c as usize] >= req {
                cnt[c as usize] -= req;
                ans[i] = c;
                ans[n-i-1] = c;
                break;
            }
        }
    }
    if n & 1 == 1 { ans[n>>1] = cnt.iter().rposition(|&x| x == 1).unwrap() as u8; }
    println!("{}", ans.into_iter().map(|x| x.to_string()).collect::<String>())
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