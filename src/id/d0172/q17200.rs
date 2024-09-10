// BOJ 17200 [Cow Evolution]
pub fn main() { read();
    let n = next::<u32>();
    let mut map = std::collections::HashMap::new();
    let mut it = 0;
    let mut v = vec![];
    for i in 0..n {
        for _ in 0..next() {
            let s = next::<String>();
            let x = *map.entry(s).or_insert_with(|| { v.push(0u32); it += 1; it - 1 });
            v[x] |= 1 << i;
        }
    }
    for i in 0..it-1 { for j in i+1..it {
        let (x, y, z) = (v[i].count_ones(), v[j].count_ones(), (v[i] | v[j]).count_ones());
        if z != x + y && z != x.max(y) { println!("no"); return; }
    }}
    println!("yes");
}

#[allow(unused_macros)] macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_macros)] macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_imports)] use println;
#[allow(unused_imports)] use print;
#[allow(unused_imports)] use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
static mut BUF: String = String::new();
static mut IT: Option<SplitAsciiWhitespace> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut *addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}