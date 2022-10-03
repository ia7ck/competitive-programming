use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    };

    let mut rep = vec![[0; 10]; n];
    for d in 1..=9 {
        rep[0][d] = d % m;
    }
    for i in 1..n {
        for d in 1..=9 {
            rep[i][d] = (rep[i - 1][d] * 10 + d)% m;
        }
    }
    for i in (0..n).rev() {
        for d in (1..=9).rev() {
            if rep[i][d] == 0 {
                let x = d.to_string().repeat(i + 1);
                println!("{}", x);
                return;
            }
        }
    }
    println!("-1");
}
