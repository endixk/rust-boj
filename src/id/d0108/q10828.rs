// BOJ 10828 [Stack]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_line(&mut s).unwrap();
    s.trim().to_string()
}

struct Stack {
    v: Vec<i32>,
}

impl Stack {
    fn new() -> Self {
        Self { v: Vec::new() }
    }

    fn push(&mut self, x: i32) {
        self.v.push(x);
    }

    fn pop(&mut self) -> i32 {
        if self.v.is_empty() {
            -1
        } else {
            self.v.pop().unwrap()
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

    fn top(&self) -> i32 {
        if self.v.is_empty() {
            -1
        } else {
            *self.v.last().unwrap()
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

    let mut stack = Stack::new();
    for _ in 0..read(&mut si).parse().unwrap() {
        let (cmd, arg) = parse(&read(&mut si));
        match cmd.as_str() {
            "push" => stack.push(arg),
            "pop" => writeln!(so, "{}", stack.pop()).unwrap(),
            "size" => writeln!(so, "{}", stack.size()).unwrap(),
            "empty" => writeln!(so, "{}", stack.empty()).unwrap(),
            "top" => writeln!(so, "{}", stack.top()).unwrap(),
            _ => panic!("Invalid command"),
        }
    }
}
