use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        l: Usize1,
        r: Usize1,
    };

    let mut a = Vec::new();
    for i in 0..n {
        if i < l {
            a.push(i + 1);
        } else if i <= r {
            a.push(r - i + l + 1);
        } else {
            a.push(i + 1);
        }
    }

    let ans = a
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", ans);
}
