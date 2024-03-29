// BOJ 6576 [Quad Tree]
fn proc(b: &mut Vec<Vec<bool>>, s: &[u8], p: &mut usize, i: usize, j: usize, k: usize) {
    *p += 1;
    if s[*p-1] == b'B' {
        for x in i..i+k { for y in j..j+k { b[x][y] = true; } }
    } else if s[*p-1] == b'Q' {
        proc(b, s, p, i, j, k/2);
        proc(b, s, p, i, j+k/2, k/2);
        proc(b, s, p, i+k/2, j, k/2);
        proc(b, s, p, i+k/2, j+k/2, k/2);
    }
}
pub fn main() { read();
    let n = next::<usize>();
    let mut b = vec![vec![false; n]; n];
    let mut p = 0;
    let s = next::<String>().into_bytes();
    proc(&mut b, &s, &mut p, 0, 0, n);

    println!("#define quadtree_width {}", n);
    println!("#define quadtree_height {}", n);
    println!("static char quadtree_bits[] = {{");
    for i in 0..n {
        for j in 0..n/8 {
            let mut x = 0;
            for k in 0..8 {
                x |= (b[i][j*8+k] as i32) << k;
            }
            print!("0x{:02x},", x);
        }
        println!();
    }
    println!("}};");
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