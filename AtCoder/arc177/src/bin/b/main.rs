use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    // 2進数っぽい？
    // 001 を作るには？
    //      A       A       A       B       B
    // 000 --> 100 --> 110 --> 111 --> 011 --> 001
    // 
    // 01010 を作るには？
    // 00000 -> ... -> 01110 ->
    // ... -> 00010 -> ...
    // ... -> 01010
    //
    // 右から 1 を作れる？
    // ひとつの 1 を作るのに ≦ 2N回程度で1e6に余裕

    let mut ans = String::new();
    for i in (0..n).rev() {
        if s[i] == '1' {
            ans.push_str("A".repeat(i + 1).as_str());
            ans.push_str("B".repeat(i).as_str());
        }
    }

    assert!(ans.len() <= 1_000_000);
    println!("{}", ans.len());
    println!("{}", ans);
}
