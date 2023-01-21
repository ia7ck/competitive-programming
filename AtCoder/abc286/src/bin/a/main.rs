use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        p: Usize1,
        q: Usize1,
        r: Usize1,
        s: Usize1,
        mut a: [u32; n],
    };

    for (i, j) in (p..=q).zip(r..=s) {
        a.swap(i, j);
    }
    
    for i in 0..n {
        print!("{}", a[i]);
        if i + 1 < n {
            print!(" ");
        } else {
            println!();
        }
    }
}
