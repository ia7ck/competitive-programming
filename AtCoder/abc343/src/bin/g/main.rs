use proconio::{input, marker::Chars};
use rolling_hash::RollingHash;

fn is_substring(s1: &Vec<char>, rh1: &RollingHash, s2: &Vec<char>, rh2: &RollingHash) -> bool {
    for j in 0..s2.len() {
        if j + s1.len() > s2.len() {
            break;
        }
        if rh1.get(0..s1.len()) == rh2.get(j..(j + s1.len())) {
            return true;
        }
    }
    false
}

fn overlap_length(s1: &Vec<char>, rh1: &RollingHash, s2: &Vec<char>, rh2: &RollingHash) -> usize {
    for j in (1..=s2.len()).rev() {
        if j <= s1.len() && rh1.get((s1.len() - j)..s1.len()) == rh2.get(0..j) {
            return j;
        }
    }
    0
}

fn main() {
    input! {
        n: usize,
        strings: [Chars; n],
    };

    let strings = {
        let hashes = strings
            .iter()
            .map(|s| RollingHash::from_iter(s.iter().map(|&c| c as u64)))
            .collect::<Vec<_>>();
        let mut new_strings = Vec::new();
        for i in 0..n {
            new_strings
                .retain(|(j, _)| !is_substring(&strings[*j], &hashes[*j], &strings[i], &hashes[i]));
            if new_strings
                .iter()
                .any(|(j, _)| is_substring(&strings[i], &hashes[i], &strings[*j], &hashes[*j]))
            {
                continue;
            }
            new_strings.push((i, strings[i].clone()));
        }
        new_strings.into_iter().map(|(_, s)| s).collect::<Vec<_>>()
    };
    let n = strings.len();
    let hashes = strings
        .iter()
        .map(|s| RollingHash::from_iter(s.iter().map(|&c| c as u64)))
        .collect::<Vec<_>>();

    let mut edges = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let len = overlap_length(&strings[i], &hashes[i], &strings[j], &hashes[j]);
            assert!(len < strings[j].len());
            edges[i].push((j, len));
        }
    }

    const INF: usize = usize::MAX;
    let mut dp = vec![vec![INF; 1 << n]; n];
    for i in 0..n {
        dp[i][1 << i] = strings[i].len();
    }
    for bits in 0..(1 << n) {
        for cur in 0..n {
            if bits >> cur & 1 == 0 {
                continue;
            }
            if dp[cur][bits] == INF {
                continue;
            }
            for &(next, len) in &edges[cur] {
                if bits >> next & 1 == 1 {
                    continue;
                }
                let new_bits = bits ^ (1 << next);
                dp[next][new_bits] =
                    dp[next][new_bits].min(dp[cur][bits] + strings[next].len() - len);
            }
        }
    }
    let mut ans = INF;
    for i in 0..n {
        ans = ans.min(dp[i][(1 << n) - 1]);
    }
    println!("{}", ans);
}
