// BOJ 6583 [Balancing Bank Accounts]
pub fn main() { read();
    for tc in 1.. {
        let (n, t) = (next::<usize>(), next::<usize>());
        if n == 0 && t == 0 { break; }

        let (mut names, mut map) = (vec![], std::collections::HashMap::new());
        for i in 0..n {
            let name = next::<String>();
            names.push(name.clone());
            map.insert(name, i);
        }

        let mut balance = vec![0; n];
        for _ in 0..t {
            let (a, b, c) = (map[&next::<String>()], map[&next::<String>()], next::<i32>());
            balance[a] -= c; balance[b] += c;
        }

        println!("Case #{}", tc);
        'x: loop {
            for i in 0..n { for j in 0..n {
                if balance[i] > 0 && balance[j] < 0 {
                    let m = balance[i].min(-balance[j]);
                    balance[i] -= m; balance[j] += m;
                    println!("{} {} {}", names[i], names[j], m);
                    continue 'x;
                }
            }}
            break;
        }
        println!();
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