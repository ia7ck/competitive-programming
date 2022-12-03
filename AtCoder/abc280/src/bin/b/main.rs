use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [i64; n],
    };

    let mut a = Vec::new();
    a.push(s[0]);
    for i in 1..n {
        let sub = a.iter().sum::<i64>();
        a.push(s[i] - sub);
    }

    for i in 0..n {
        print!("{}", a[i]);
        if i + 1 < n {
            print!(" ");
        }
    }
    println!();
}
