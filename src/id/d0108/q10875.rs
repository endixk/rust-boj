// BOJ 10875 [Snake]
// Supported by GitHub Copilot

use std::io::{self, Read};

fn read<T>(si: &mut T) -> String where T: Read {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

fn next<T>(it: &mut std::str::SplitAsciiWhitespace) -> T where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {
    it.next().unwrap().parse().unwrap()
}

struct Line {
    axis: bool, // true: x, false: y
    s: i64,
    e: i64,
    i: i64,
}
impl Line {
    fn isect(&self, other: &Line) -> Option<i64> {
        if self.axis == other.axis {
            if self.i != other.i { return None }
            let (ds, de) = ((self.s - other.s).abs(), (self.s - other.e).abs());
            let t = if ds < de { other.s } else { other.e };
            if self.s <= self.e {
                if self.s <= t && t <= self.e { return Some(t - self.s) }
            } else {
                if self.e <= t && t <= self.s { return Some(self.s - t) }
            }
        } else {
            if other.s <= other.e && (self.i < other.s || other.e < self.i) { return None }
            if other.e <= other.s && (self.i < other.e || other.s < self.i) { return None }
            if self.s <= self.e {
                if self.s <= other.i && other.i <= self.e { return Some(other.i - self.s) }
            } else {
                if self.e <= other.i && other.i <= self.s { return Some(self.s - other.i) }
            }
        }
        None
    }
}

fn turn(x: bool, p: bool, dir: char) -> (bool, bool) {
    match dir {
        'L' => (!x, if x { p } else { !p }),
        'R' => (!x, if x { !p } else { p }),
        _ => unreachable!()
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (l, n) = (next::<i64>(&mut it), next::<i64>(&mut it));
    if n == 0 { println!("{}", l+1); return; }

    let mut lines = Vec::new();
    lines.push(Line { axis: true, s: -l, e: l, i: l+1 });
    lines.push(Line { axis: true, s: -l, e: l, i: -l-1 });
    lines.push(Line { axis: false, s: -l, e: l, i: l+1 });
    lines.push(Line { axis: false, s: -l, e: l, i: -l-1 });
    lines.push(Line { axis: true, s: 0, e: 0, i: 0 });

    let (mut x, mut p) = (true, true);
    let (mut i, mut j) = (0, 0);
    let mut ans = 0;
    let mut flag = false;
    for _ in 0..n {
        let (t, dir) = (next::<i64>(&mut it), next::<char>(&mut it));
        let line = match (x, p) {
            (true, true) => Line { axis: true, s: i+1, e: i+t, i: j },
            (true, false) => Line { axis: true, s: i-1, e: i-t, i: j },
            (false, true) => Line { axis: false, s: j+1, e: j+t, i },
            (false, false) => Line { axis: false, s: j-1, e: j-t, i },
        };
        if let Some(d) = lines.iter().filter_map(|l| line.isect(l)).min() {
            ans += d;
            flag = true;
            break;
        }
        (x, p) = turn(x, p, dir);
        (i, j) = match line.axis {
            true => (line.e, line.i),
            false => (line.i, line.e)
        };
        lines.push(line);
        ans += t;
    }
    if !flag {
        let line = match (x, p) {
            (true, true) => Line { axis: true, s: i+1, e: i+3*l, i: j },
            (true, false) => Line { axis: true, s: i-1, e: i-3*l, i: j },
            (false, true) => Line { axis: false, s: j+1, e: j+3*l, i },
            (false, false) => Line { axis: false, s: j-1, e: j-3*l, i },
        };
        ans += lines.iter().filter_map(|l| line.isect(l)).min().unwrap();
    }

    println!("{}", ans+1);
}
