// BOJ 10845 [Queue]
// Supported by GitHub Copilot

use std::collections::VecDeque;
use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_line(&mut s).unwrap();
    s.trim().to_string()
}

struct Queue {
    v: VecDeque<i32>,
}

impl Queue {
    fn new() -> Self {
        Self { v: VecDeque::new() }
    }

    fn push(&mut self, x: i32) {
        self.v.push_back(x);
    }

    fn pop(&mut self) -> i32 {
        if self.v.is_empty() {
            -1
        } else {
            self.v.pop_front().unwrap()
        }
    }

    fn size(&self) -> usize {
        self.v.len()
    }

    fn empty(&self) -> i32 {
        if self.v.is_empty() {
            1
        } else {
            0
        }
    }

    fn front(&self) -> i32 {
        if self.v.is_empty() {
            -1
        } else {
            *self.v.front().unwrap()
        }
    }

    fn back(&self) -> i32 {
        if self.v.is_empty() {
            -1
        } else {
            *self.v.back().unwrap()
        }
    }
}

fn parse(s: &str) -> (String, i32) {
    let mut it = s.split_whitespace();
    let cmd = it.next().unwrap().to_string();
    let arg = it.next().unwrap_or("0").parse().unwrap();
    (cmd, arg)
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let mut queue = Queue::new();
    for _ in 0..read(&mut si).parse().unwrap() {
        let (cmd, arg) = parse(&read(&mut si));
        match cmd.as_str() {
            "push" => queue.push(arg),
            "pop" => writeln!(so, "{}", queue.pop()).unwrap(),
            "size" => writeln!(so, "{}", queue.size()).unwrap(),
            "empty" => writeln!(so, "{}", queue.empty()).unwrap(),
            "front" => writeln!(so, "{}", queue.front()).unwrap(),
            "back" => writeln!(so, "{}", queue.back()).unwrap(),
            _ => panic!("Unknown command: {}", cmd),
        }
    }
}