use next_permutation::NextPermutation;
use proconio::input;
use std::collections::HashSet;

fn dfs(i: usize, s_max: usize, a: &mut Vec<usize>, result: &mut Vec<Vec<usize>>) {
    if i == a.len() {
        result.push(a.clone());
        return;
    }
    let s = a.iter().sum::<usize>();
    assert!(s <= s_max);
    dfs(i + 1, s_max, a, result);
    for _ in 0..(s_max - s) {
        a[i] += 1;
        dfs(i + 1, s_max, a, result);
    }
    for _ in 0..(s_max - s) {
        a[i] -= 1;
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ss: [String; n],
        tt: [String; m],
    };

    let s_len_sum = ss.iter().map(|s| s.len()).sum::<usize>();
    let mut pattern = Vec::new();
    dfs(0, 16 - s_len_sum, &mut vec![1; n - 1], &mut pattern);

    let tt = tt.into_iter().collect::<HashSet<_>>();

    let mut ord: Vec<usize> = (0..n).collect();
    loop {
        for pat in &pattern {
            let mut s = Vec::new();
            for i in 0..n {
                s.push(ss[ord[i]].clone());
                if i + 1 < n {
                    s.push("_".repeat(pat[i]));
                }
            }
            let s = s.join("");
            if s.len() >= 3 && s.len() <= 16 && !tt.contains(&s) {
                println!("{}", s);
                return;
            }
        }

        if !ord.next_permutation() {
            break;
        }
    }

    println!("-1");
}
