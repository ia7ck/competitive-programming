use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    };

    if let Some(i) = (0..n).find(|&i| s[i].iter().all(|&ch| ch == 'x')) {
        println!("Faucet {}", i + 1);
        return;
    }

    let j = (0..m).find(|&j| (0..n).all(|i| s[i][j] == 'x')).unwrap();
    println!("Drain {}", j + 1);
}
