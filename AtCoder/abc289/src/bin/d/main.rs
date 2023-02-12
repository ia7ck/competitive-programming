use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        m: usize,
        b: [usize; m],
        x: usize,
    };

    let mut ban = vec![false; x + 1];
    for b in b {
        ban[b] = true;
    }
    let mut dp = vec![false; x + 1];
    dp[0] = true;
    for i in 0..x {
        if dp[i] == false {
            continue;
        }
        for &a in &a {
            if i + a <= x && ban[i + a] == false {
                dp[i + a] = true;
            }
        }
    }
    if dp[x] {
        println!("Yes");
    } else {
        println!("No");
    }
}
