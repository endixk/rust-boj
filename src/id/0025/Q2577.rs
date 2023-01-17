// BOJ 2577 [Number of Numbers]
// Supported by GitHub Copilot

use std::io;

fn main() {
    let mut num = 1;
    for _ in 0..3 {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        num *= buf.trim().parse::<i32>().unwrap();
    }
    let mut cnt = [0; 10];
    while num > 0 {
        cnt[(num % 10) as usize] += 1;
        num /= 10;
    }
    for i in 0..10 {
        println!("{}", cnt[i]);
    }
}
