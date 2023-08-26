pub fn main() {
    println!("{}", std::io::stdin().lines().skip(1).filter(|s| {
        let mut c = s.as_ref().unwrap().chars().collect::<Vec<_>>();
        c.dedup();
        c.sort_unstable();
        (1..c.len()).all(|i| c[i - 1] != c[i])
    }).count());
}