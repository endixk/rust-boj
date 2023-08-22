// BOJ 12928 [Length of Tree Paths]
// Supported by GitHub Copilot

pub fn main() {
    let v = std::io::stdin().lines().next().unwrap().unwrap()
        .split_ascii_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let (n, s) = (v[0], v[1]);
    if n < 3 { println!("0"); return; }

    let mut dp = vec![vec![false; n]; s+1];
    let mut dv = vec![false; s+1];
    dp[1][1] = true; dp[1][2] = true; dv[1] = true;
    for _ in 3..n {
        let mut tp = vec![vec![false; n]; s+1];
        let mut tv = vec![false; s+1];
        for i in 0..s {
            if !dv[i] { continue; }
            for j in 0..n {
                if dp[i][j] && i+j <= s {
                    tp[i+j][j+1] = true;
                    tp[i+j][1] = true;
                    tv[i+j] = true;
                }
            }
        }
        dp = tp;
        dv = tv;
    }
    if dp[s].iter().any(|&x| x) { println!("1"); }
    else { println!("0"); }
}
