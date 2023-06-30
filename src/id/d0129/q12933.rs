// BOJ 12933 [Quack]
// Supported by GitHub Copilot

use std::io::{self, BufRead};

const QUACK: [char; 5] = ['q', 'u', 'a', 'c', 'k'];
#[derive(Copy, Clone)]
struct Duck {
    expect: usize,
    done: bool,
}
impl Duck {
    fn new() -> Self {
        Self {
            expect: 0,
            done: false,
        }
    }
    fn accept(&mut self, c: char) -> bool {
        if c == QUACK[self.expect] {
            if self.expect == 4 { self.done = true; }
            self.expect = (self.expect + 1) % 5;
            true
        } else {
            false
        }
    }
}

pub fn main() {
    let quack = io::stdin().lock().lines().next().unwrap().unwrap();
    let mut ducks = vec![Duck::new(); 2500];
    for c in quack.chars() {
        let mut flag = true;
        for duck in ducks.iter_mut() {
            if duck.accept(c) { flag = false; break; }
        }
        if flag { println!("-1"); return; }
    }

    if ducks.iter().any(|duck| duck.expect != 0) {
        println!("-1");
        return
    }
    println!("{}", ducks.iter().filter(|duck| duck.done).count());
}
