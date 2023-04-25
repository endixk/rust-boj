// BOJ 10866 [Deque]
// Supported by GitHub Copilot

use std::collections::VecDeque;
use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_line(&mut s).unwrap();
    s.trim().to_string()
}

struct Deque {
    v: VecDeque<i32>,
}

impl Deque {
    fn new() -> Self {
        Self { v: VecDeque::new() }
    }

    fn push_front(&mut self, x: i32) {
        self.v.push_front(x);
    }

    fn push_back(&mut self, x: i32) {
        self.v.push_back(x);
    }

    fn pop_front(&mut self) -> i32 {
        if self.v.is_empty() {
            -1
        } else {
            self.v.pop_front().unwrap()
        }
    }

    fn pop_back(&mut self) -> i32 {
        if self.v.is_empty() {
            -1
        } else {
            self.v.pop_back().unwrap()
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

    let mut deque = Deque::new();
    for _ in 0..read(&mut si).parse().unwrap() {
        let (cmd, arg) = parse(&read(&mut si));
        match cmd.as_str() {
            "push_front" => deque.push_front(arg),
            "push_back" => deque.push_back(arg),
            "pop_front" => writeln!(so, "{}", deque.pop_front()).unwrap(),
            "pop_back" => writeln!(so, "{}", deque.pop_back()).unwrap(),
            "size" => writeln!(so, "{}", deque.size()).unwrap(),
            "empty" => writeln!(so, "{}", deque.empty()).unwrap(),
            "front" => writeln!(so, "{}", deque.front()).unwrap(),
            "back" => writeln!(so, "{}", deque.back()).unwrap(),
            _ => unreachable!(),
        }
    }
}