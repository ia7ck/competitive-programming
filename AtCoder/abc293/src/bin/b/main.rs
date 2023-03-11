use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    };

    let mut called = vec![false; n];
    for i in 0..n {
        if called[i] == false {
            called[a[i]] = true;
        }
    }

    let mut ans = Vec::new();
    for i in 0..n {
        if called[i] == false {
            ans.push(i + 1);
        }
    }
    println!("{}", ans.len());
    for i in 0..ans.len() {
        print!("{}", ans[i]);
        if i + 1 < ans.len() {
            print!(" ");
        } else {
            print!("\n");
        }
    }
}
