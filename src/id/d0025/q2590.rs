// BOJ 2590 [Colored Papers]
// Supported by GitHub Copilot

use std::io::BufRead;
pub fn main() {
    let v = std::io::stdin().lock().lines()
        .map(|x| x.unwrap().parse::<i32>().unwrap()).collect::<Vec<_>>();
    let mut a = v[3] + v[4] + v[5];
    let mut o = v[3] * 20 + v[4] * 11;
    let mut t = v[3] * 5;
    a += v[2] / 4 + if v[2] % 4 > 0 { 1 } else { 0 };
    match v[2] % 4 {
        1 => { o += 27; t += 5; },
        2 => { o += 18; t += 3; },
        3 => { o += 9; t += 1; },
        _ => (),
    }
    if t > v[1] { o -= v[1] * 4; }
    else {
        let k = (v[1] - t) / 9 + if (v[1] - t) % 9 > 0 { 1 } else { 0 };
        a += k;
        o += k * 36 - v[1] * 4;
    }
    if o < v[0] {
        a += (v[0] - o) / 36 + if (v[0] - o) % 36 > 0 { 1 } else { 0 };
    }
    println!("{}", a);
}
