// BOJ 6562 [Etaoin Shrdlu]
pub fn main() {
    loop {
        let n = next().parse::<u8>().unwrap();
        if n == 0 { break; }

        let mut map = std::collections::HashMap::<u16, usize>::new();
        let mut dgr = Vec::<(u16, i32)>::new();
        let mut sum = 0;
        let mut s = String::new();
        for _ in 0..n { s += next().as_str(); }

        let s = s.into_bytes();
        for i in 0..s.len()-1 {
            let d = (s[i] as u16) << 8 | s[i+1] as u16;
            let k = *map.entry(d).or_insert_with(|| { dgr.push((d, 0)); dgr.len() - 1 });
            dgr[k].1 += 1;
            sum += 1;
        }

        dgr.sort_unstable_by(|&x, &y| y.1.cmp(&x.1).then(x.0.cmp(&y.0)));
        for i in 0..5.min(dgr.len()) {
            println!("{}{} {} {:.6}", (dgr[i].0 >> 8) as u8 as char, (dgr[i].0 & 0xff) as u8 as char, dgr[i].1, dgr[i].1 as f64 / sum as f64);
        }
        println!();
    }
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
use println;
use std::{io::*, cell::*, str::*, fmt::Debug};
static mut BUF: String = String::new();
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn next() -> String { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_line(&mut *std::ptr::addr_of_mut!(BUF)).unwrap());
    BUF.pop();
    BUF.clone()
}}