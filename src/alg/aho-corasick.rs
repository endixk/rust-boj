struct TrieNode {
    next: Vec<usize>,
    fail: usize,
    end: bool,
}
impl TrieNode {
    fn new(size: usize) -> Self {
        Self {
            next: vec![0; size],
            fail: 0,
            end: false,
        }
    }
}

fn hash(base: char) -> usize {
    unimplemented!();
}

fn insert(trie: &mut Vec<TrieNode>, s: &str, size: usize) {
    let mut cur = 0;
    for c in s.chars() {
        let i = hash(c);
        if trie[cur].next[i] == 0 {
            trie.push(TrieNode::new(size));
            trie[cur].next[i] = trie.len() - 1;
        }
        cur = trie[cur].next[i];
    }
    trie[cur].end = true;
}

fn build(trie: &mut Vec<TrieNode>, size: usize) {
    let mut q = VecDeque::new();
    q.push_back(0);
    while let Some(cur) = q.pop_front() {
        for i in 0..size {
            let next = trie[cur].next[i];
            if next == 0 { continue; }

            if cur == 0 {
                trie[next].fail = 0;
            } else {
                let mut f = trie[cur].fail;
                while f != 0 && trie[f].next[i] == 0 {
                    f = trie[f].fail;
                }
                if trie[f].next[i] != 0 {
                    f = trie[f].next[i];
                }
                trie[next].fail = f;
            }

            trie[next].end |= trie[trie[next].fail].end;
            q.push_back(next);
        }
    }
}

fn find(trie: &Vec<TrieNode>, s: &str) -> usize {
    let mut cur = 0;
    let mut cnt = 0;
    for c in s.chars() {
        let i = hash(c);
        while cur != 0 && trie[cur].next[i] == 0 {
            cur = trie[cur].fail;
        }
        if trie[cur].next[i] != 0 {
            cur = trie[cur].next[i];
        }
        if trie[cur].end {
            cnt += 1;
        }
    }
    cnt
}