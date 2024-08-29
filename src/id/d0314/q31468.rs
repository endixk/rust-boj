// BOJ 31468 [Triangular Chocolate Packaging (Bitter)]
const P9: [&str; 9] = [
    "R", "RR", "RBB", "RRBR", "RBBRR", "RRBBBR", "BBBBBRR", "RBRBRBBR", "RRRRRRBRR",
];
const P11: [&str; 11] = [
    "R", "RR", "RBB", "RRBR", "RBBRR", "RRBBBR", "BBBBBRR", "RBRBRBBR", "RRRRRRBRR",
    "RBBRBBRBBR", "RRBRRBRRBRR",
];
const R11: [&str; 11] = [
    "BBRBBRBBRBB", "BRRBRRBRRB",
    "BBBBBBRBB", "BRBRBRRB", "RRRRRBB", "BBRRRB", "BRRBB", "BBRB", "BRR", "BB", "B",
];
const P12: [&str; 12] = [
    "R", "RR", "RBB", "RRBR", "RBBRR", "RRBBBR", "BBBBBRR", "RBRBRBBR", "RRRRRRBRR",
    "BBBBBBBBBB", "RBRBRBRBRBR", "RRRRRRRRRRRR",
];
fn put(t: i8, i: usize) {
    print!("{}", match t {
        -11 => R11[i], 9 => P9[i], 11 => P11[i], 12 => P12[i], _ => panic!(),
    });
}
fn gen(x: usize) {
    for i in 0..x {
        for k in 0..11 {
            for _ in 0..i { put(12, k); put(-11, k); }
            put(12, k);
            println!();
        }
        for _ in 0..=i { put(12, 11); }
        println!();
    }
}
fn add(x: usize, r: usize) {
    if r == 2 {
        for _ in 0..x { print!("RBBRBBRBBRBB"); }
        println!("R");
        for _ in 0..x { print!("RRBRRBRRBRRB"); }
        println!("RR");
    } else if r == 9 {
        for k in 0..9 {
            for _ in 0..x { put(12, k); put(-11, k); }
            put(12, k);
            println!();
        }
    } else if r == 11 {
        for k in 0..11 {
            for _ in 0..x { put(11, k); put(-11, k); }
            put(11, k);
            println!();
        }
    }
}
pub fn main() {
    let n = stdin().lock().lines().next().unwrap().unwrap().parse::<usize>().unwrap();
    gen(n / 12);
    add(n / 12, n % 12);
}

#[allow(unused_macros)] macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_macros)] macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_imports)] use println;
#[allow(unused_imports)] use print;
#[allow(unused_imports)] use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
thread_local! {
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}