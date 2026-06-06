use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
        b: [Usize1; n],
    };

    let mut ans = true;
    for i in 0..n {
        ans &= b[a[i]] == i;
    }

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
