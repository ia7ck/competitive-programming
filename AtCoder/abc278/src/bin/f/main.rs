use proconio::{input, marker::Chars};

fn judge(n: usize, last: usize, used: usize, s: &[Vec<char>], memo: &mut Vec<Vec<Option<bool>>>) -> bool {
    if used == (1 << n) - 1 {
        return false;
    }
    if let Some(result) = memo[last][used] {
        return result;
    }
    let mut win = false;
    for i in 0..n {
        if last == n || (used >> i & 1 == 0 && s[last][s[last].len() - 1] == s[i][0]){
            if !judge(n, i, used ^ (1 << i), s, memo) {
                win = true;
            }
        }
    }
    memo[last][used] = Some(win);
    win
}

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };

    let mut memo = vec![vec![None; 1 << n]; n + 1];

    let ans = judge(n, n, 0, &s, &mut memo);
    if ans {
        println!("First");
    } else {
        println!("Second");
    }
}
