// BOJ 27718 [Line Segment Intersection EX]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

fn next<T>(it: &mut std::str::SplitAsciiWhitespace) -> T where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {
    it.next().unwrap().parse().unwrap()
}

fn ccw(x1: i64, y1: i64, x2: i64, y2: i64, x3: i64, y3: i64) -> i32 {
    let ret = (x2 - x1) * (y3 - y1) - (y2 - y1) * (x3 - x1);
    if ret > 0 { 1 } else if ret < 0 { -1 } else { 0 }
}

struct Line { x1: i64, y1: i64, x2: i64, y2: i64, ymin: i64, ymax: i64 }
impl Line {
    fn new() -> Self {
        Self { x1: 0, y1: 0, x2: 0, y2: 0, ymin: 0, ymax: 0 }
    }
    fn intersect(&self, l: &Line) -> u8 {
        let ccw1 = ccw(self.x1, self.y1, self.x2, self.y2, l.x1, l.y1);
        let ccw2 = ccw(self.x1, self.y1, self.x2, self.y2, l.x2, l.y2);
        let ccw3 = ccw(l.x1, l.y1, l.x2, l.y2, self.x1, self.y1);
        let ccw4 = ccw(l.x1, l.y1, l.x2, l.y2, self.x2, self.y2);

        if ccw1 == 0 && ccw2 == 0 && ccw3 == 0 && ccw4 == 0 {
            let xp = self.x1.max(l.x1);
            let xq = self.x2.min(l.x2);
            let yp = self.ymin.max(l.ymin);
            let yq = self.ymax.min(l.ymax);

            if xp < xq || yp < yq { 3 }
            else if xp == xq && yp == yq { 1 }
            else { 0 }
        } else if ccw1 * ccw2 <= 0 && ccw3 * ccw4 <= 0 {
            if ccw1 * ccw2 == 0 || ccw3 * ccw4 == 0 { 1 }
            else { 2 }
        } else {
            0
        }
    }
}
impl Clone for Line {
    fn clone(&self) -> Self {
Self { x1: self.x1, y1: self.y1, x2: self.x2, y2: self.y2, ymin: self.ymin, ymax: self.ymax }
    }
}
impl Copy for Line {}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let n = next(&mut it);
    let mut lines = [Line::new(); 2002];
    for i in 0..n {
        let (x1, y1, x2, y2) = (next(&mut it), next(&mut it), next(&mut it), next(&mut it));
        if x1 < x2 {
            if y1 < y2 {
                lines[i] = Line { x1, y1, x2, y2, ymin: y1, ymax: y2 };
            } else {
                lines[i] = Line { x1, y1, x2, y2, ymin: y2, ymax: y1 };
            }
        } else {
            if y1 < y2 {
                lines[i] = Line { x1: x2, y1: y2, x2: x1, y2: y1, ymin: y1, ymax: y2 };
            } else {
                lines[i] = Line { x1: x2, y1: y2, x2: x1, y2: y1, ymin: y2, ymax: y1 };
            }
        }
    }

    for lx in &lines[..n] {
        for ly in &lines[..n] {
            write!(so, "{}", lx.intersect(&ly)).unwrap();
        }
        writeln!(so).unwrap();
    }
}
