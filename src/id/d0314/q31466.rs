// BOJ 31466 [Chocolate Frogs and Stepping Stones]
pub fn main() {
    let n = stdin().lock().lines().next().unwrap().unwrap().parse::<usize>().unwrap();
    if n == 1 { println!("1\n0 2"); return; }

    if n & 1 == 0 {
        println!("{}", n * (n/2 + 2) - 1);
        for i in 1..n {
            if i & 1 == 1 { println!("{} {}", 0, 1); }
            else {
                for j in (0..i).step_by(2) {
                    println!("{} {}", j, j+2);
                }
                println!("{} {}", i, i+1);
                println!("{} {}", 1, 2);
                for j in (2..i).step_by(2) {
                    println!("{} {}", j, j+2);
                }
                println!("{} {}", i, i+1);
            }
        }
        for i in (0..n).step_by(2) { println!("{} {}", i, i+2); }
        println!("{} {}", n, n+1);
        println!("{} {}", 1, 2);
        for i in (2..n).step_by(2) { println!("{} {}", i, i+2); }
        println!("{} {}", n, n+1);
        for i in (1..n).rev() {
            if i & 1 == 0 {
                println!("{} {}", i+1, i+2);
                for j in (i+1..n).step_by(2) { println!("{} {}", j, j+2); }
                println!("{} {}", i+2, i+3);
                for j in (i+3..n).step_by(2) { println!("{} {}", j, j+2); }
            }
        }
    } else {
        println!("{}", (n-1)*(n/2+3));
        for i in 1..n {
            if i & 1 == 1 { println!("{} {}", 0, 1); }
            else {
                for j in (0..i).step_by(2) {
                    println!("{} {}", j, j+2);
                }
                println!("{} {}", i, i+1);
                if i == n-1 { break; }
                println!("{} {}", 1, 2);
                for j in (2..i).step_by(2) {
                    println!("{} {}", j, j+2);
                }
                println!("{} {}", i, i+1);
            }
        }
        for i in (0..n).step_by(2) { println!("{} {}", i, i+2); }
        println!("{} {}", n, n+1);
        println!("{} {}", 1, 2);
        for i in (2..n).step_by(2) { println!("{} {}", i, i+2); }
        for i in (1..n-1).rev() {
            if i & 1 == 0 {
                println!("{} {}", i+1, i+2);
                for j in (i+1..n).step_by(2) { println!("{} {}", j, j+2); }
                println!("{} {}", n, n+1);
                for j in (i+2..n+1).step_by(2) { println!("{} {}", j, j+2); }
            }
        }
    }
}

#[allow(unused_macros)] macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_imports)] use println;
#[allow(unused_imports)] use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
thread_local! {
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}