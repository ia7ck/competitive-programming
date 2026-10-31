use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [Chars; N],
    };

    let mut ans = vec![N];
    for i in 0..N {
        let c = s[i].iter().filter(|&&ch| ch == 'o').count();
        ans.push(N - c);
    }
    for j in 0..N {
        let c = (0..N).filter(|&i| s[i][j] == 'o').count();
        ans.push(N - c);
    }
    let c = (0..N).filter(|&i| s[i][i] == 'o').count();
    ans.push(N - c);
    let c = (0..N).filter(|&i| s[i][N - i - 1] == 'o').count();
    ans.push(N - c);

    let ans = ans.into_iter().min().unwrap();
    println!("{}", ans);
}

const N: usize = 5;
