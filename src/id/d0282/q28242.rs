// BOJ 28242 [Math Teacher's Concern]
// Supported by GitHub Copilot

use std::io::{self, BufRead};

pub fn main() {
    let n = io::stdin().lock().lines().next().unwrap().unwrap().parse::<i32>().unwrap();
    let mut i = 1;
    while i*i <= n {
        if n % i == 0 {
            let (a, c) = (i, n/i);
            let mut j = 1;
            while j*j <= n+2 {
                if (n+2) % j == 0 {
                    let (b, d) = (j, (n+2)/j);
                    // a, -b, c, d
                    if a * d - b * c == n + 1 {
                        println!("{} {} {} {}", a, -b, c, d);
                        return;
                    }
                    // a, b, c, -d
                    if b * c - a * d == n + 1 {
                        println!("{} {} {} {}", a, b, c, -d);
                        return;
                    }
                    // b, -a, c, d
                    if b * d - a * c == n + 1 {
                        println!("{} {} {} {}", b, -a, c, d);
                        return;
                    }
                    // b, a, c, -d
                    if a * c - b * d == n + 1 {
                        println!("{} {} {} {}", b, a, c, -d);
                        return;
                    }
                }
                j += 1;
            }
        }
        i += 1;
    }
    println!("-1");
}
