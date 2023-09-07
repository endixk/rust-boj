#[inline]
fn digit(i: u64) -> u32 {
    let (mut r, mut n) = (0, i);
    while n > 0 { r += 1; n /= 10; }
    r
}
pub fn main() {
    let v = std::io::stdin().lines().next().unwrap().unwrap()
        .split_ascii_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();
    let mut ans = 1 % v[1];
    for i in 2..=v[0] { ans = (ans * 10u64.pow(digit(i)) + i) % v[1]; }
    println!("{}", ans);
}