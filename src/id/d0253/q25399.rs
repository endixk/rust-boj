// BOJ 25399 [Lagrangian Subtraction]
use std::io::BufRead;
fn square(x: i64) -> Option<i64> {
    let k = ((x as f64).sqrt() + 1e-9) as i64;
    if k * k == x { Some(k) } else { None }
}
fn print(v: &[i64], neg: bool) {
    println!("{}", v.len());
    for &x in v {
        let mut sign = x < 0;
        if neg { sign = !sign; }
        println!("{} {}", if sign { '-' } else { '+' }, x.abs());
    }
}
fn odd(x: i64) -> (i64, i64) { (x + 1 >> 1, x - 1 >> 1)}
fn mul4(x: i64) -> (i64, i64) { ((x >> 2) + 1, (x >> 2) - 1) }
pub fn main() {
    let n = std::io::stdin().lock().lines().next().unwrap().unwrap().parse::<i64>().unwrap();
    if n == 0 { println!("3\n+ 5\n- 4\n- 3"); return; }
    let neg = n < 0; let n = n.abs();
    if n == 2 { print(&[6, -5, -3], neg); return; }
    if let Some(x) = square(n) { print(&[x], neg); }
    else if n & 1 == 1 {
        let (a, b) = odd(n);
        print(&[a, -b], neg);
    } else if n % 4 == 0 {
        let (a, b) = mul4(n);
        print(&[a, -b], neg);
    } else {
        for i in 1.. {
            if i*i*2 > n { break; }
            if let Some(x) = square(n - i*i) {
                print(&[i, x], neg);
                return;
            }
        }
        let (a, b) = odd(n+1);
        print(&[a, -b, -1], neg);
    }
}