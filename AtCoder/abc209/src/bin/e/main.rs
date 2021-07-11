use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let words: Vec<Vec<char>> = (0..n).map(|_| rd.get_chars()).collect();

    let k = 52 * 52 * 52;
    let mut rev_graph = vec![vec![]; k];
    let mut in_deg = vec![0; k];
    for w in &words {
        rev_graph[encode(&w[w.len() - 3..])].push(encode(&w[..3]));
        in_deg[encode(&w[..3])] += 1;
    }
    let mut ans = vec![None; k];
    use std::collections::VecDeque;
    let mut que = VecDeque::new();
    for v in 0..k {
        if in_deg[v] == 0 {
            ans[v] = Some(false);
            que.push_back(v);
        }
    }
    while let Some(u) = que.pop_front() {
        for &v in &rev_graph[u] {
            in_deg[v] -= 1;
            if ans[v].is_some() {
                continue;
            }
            match ans[u] {
                Some(judge) if !judge => {
                    ans[v] = Some(true);
                    que.push_back(v);
                }
                _ => {
                    if in_deg[v] == 0 {
                        ans[v] = Some(false);
                        que.push_back(v);
                    }
                }
            }
        }
    }
    for w in &words {
        match ans[encode(&w[w.len() - 3..])] {
            None => {
                println!("Draw");
            }
            Some(judge) => {
                if judge {
                    println!("Aoki");
                } else {
                    println!("Takahashi");
                }
            }
        }
    }
}

fn encode(s: &[char]) -> usize {
    assert_eq!(s.len(), 3);
    let mut r = 0;
    for &c in s {
        r *= 52;
        if c.is_ascii_lowercase() {
            r += c as usize - 'a' as usize;
        } else {
            r += 26 + c as usize - 'A' as usize;
        }
    }
    assert!(r <= 52 * 52 * 52, "r = {}, s = {:?}", r, s);
    r
}
