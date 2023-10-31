// BOJ 2504 [Calculating Parentheses]
// Supported by GitHub Copilot

fn valid(s: &String) -> bool {
    let mut st = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '[' => st.push(c),
            ')' => {
                if st.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if st.pop() != Some('[') {
                    return false;
                }
            }
            _ => unreachable!(),
        }
    }
    st.is_empty()
}

fn dfs(adj: &Vec<Vec<usize>>, val: &Vec<usize>, cur: usize) -> usize {
    let mut ret = 0;
    for &nxt in adj[cur].iter() {
        ret += dfs(adj, val, nxt);
    }
    if ret == 0 { val[cur] } else { ret * val[cur] }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = std::io::stdin().lines().next().unwrap()?;
    if !valid(&s) {
        println!("0");
        return Ok(());
    }

    let mut adj = vec![vec![]];
    let mut par = vec![0];
    let mut val = vec![1];
    let mut id = 1;
    let mut cur = 0;
    for c in s.chars() {
        match c {
            '(' => {
                adj[cur].push(id);
                par.push(cur);
                val.push(2);
                adj.push(vec![]);
                cur = id;
                id += 1;
            },
            '[' => {
                adj[cur].push(id);
                par.push(cur);
                val.push(3);
                adj.push(vec![]);
                cur = id;
                id += 1;
            },
            ')' | ']' => { cur = par[cur]; },
            _ => unreachable!(),
        }
    }

    println!("{}", dfs(&adj, &val, 0));

    Ok(())
}
