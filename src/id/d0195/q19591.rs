// BOJ 19591 [Weird Calculator]
// Supported by GitHub Copilot

use std::io::{self, BufRead};
use std::collections::VecDeque;

fn pri(op: char) -> u8 {
    match op {
        '+' | '-' => 0,
        '*' | '/' => 1,
        _ => 2,
    }
}
fn calc(a: i64, b: i64, op: char) -> i64 {
    match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => a / b,
        _ => 0,
    }
}
fn cmp(a1: i64, b1: i64, a2: i64, b2: i64, op1: char, op2: char) -> bool {
    let (p1, p2) = (pri(op1), pri(op2));
    if p1 != p2 { return p1 > p2; }

    let (c1, c2) = (calc(a1, b1, op1), calc(a2, b2, op2));
    if c1 != c2 { return c1 > c2; }

    true
}

pub fn main() {
    let s = io::stdin().lock().lines().next().unwrap().unwrap();
    let neg = s.starts_with('-');
    let s = s.trim_start_matches('-').to_string();

    let mut vals = VecDeque::new();
    let mut ops = VecDeque::new();
    let mut num = String::new();
    for c in s.chars() {
        if c.is_digit(10) {
            num.push(c);
        } else {
            vals.push_back(num.parse::<i64>().unwrap());
            num.clear();
            ops.push_back(c);
        }
    }
    vals.push_back(num.parse::<i64>().unwrap());
    if neg { vals[0] *= -1; }

    if ops.len() == 0 {
        println!("{}", vals[0]);
        return;
    }
    while ops.len() > 1 {
        let (a1, b1) = (vals[0], vals[1]);
        let (a2, b2) = (vals[vals.len() - 2], vals[vals.len() - 1]);
        let (op1, op2) = (ops[0], ops[ops.len() - 1]);
        if cmp(a1, b1, a2, b2, op1, op2) {
            let (a, b) = (vals.pop_front().unwrap(), vals.pop_front().unwrap());
            let op = ops.pop_front().unwrap();
            vals.push_front(calc(a, b, op));
        } else {
            let (b, a) = (vals.pop_back().unwrap(), vals.pop_back().unwrap());
            let op = ops.pop_back().unwrap();
            vals.push_back(calc(a, b, op));
        }
    }
    println!("{}", calc(vals[0], vals[1], ops[0]));
}
