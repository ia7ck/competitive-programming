use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [u64; n],
        mut b: [u64; m],
    };

    a.sort();
    b.sort();

    let mut i = 0;
    let mut ans = Vec::new();
    for j in 0..m {
        while i < n && a[i] < b[j] {
            i += 1;
        }
        if i < n {
            ans.push(i);
            i += 1;
        }
    }
    if ans.len() == m {
        let mut total = 0;
        for i in ans {
            total += a[i];
        }
        println!("{}", total);
    } else {
        println!("-1");
    }
}
