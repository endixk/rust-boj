pub fn main() {
    let s = std::io::stdin().lines().skip(1).next().unwrap().unwrap();
    let s = s.as_bytes();
    for i in 1..s.len() {
        let mut c = 0;
        for j in 0..i {
            if s[j] != s[s.len()-i+j] { c += 1; }
            if c > 1 { break; }
        }
        if c == 1 { println!("YES"); return; }
    }
    println!("NO");
}
