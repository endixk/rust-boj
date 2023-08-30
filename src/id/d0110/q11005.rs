use std::io::BufRead;
pub fn main() {
    let mut v = std::io::stdin().lock().lines().next().unwrap().unwrap()
        .split_ascii_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>();
    let mut r = Vec::new();
    while v[0] > 0 { r.push((v[0] % v[1]) as u8); v[0] /= v[1]; }
    r.iter().rev().for_each(|&x| print!("{}", if x < 10 { x + b'0' } else { x - 10 + b'A' } as char));
}
