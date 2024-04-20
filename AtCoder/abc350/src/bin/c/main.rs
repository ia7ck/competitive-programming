use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        mut a: [Usize1; n],
    };

    let mut pos = vec![0; n];
    for i in 0..n {
        pos[a[i]] = i;
    }
    let mut ans = Vec::new();
    for i in 0..(n - 1) {
        if a[i] == i {
            continue;
        }
        let j = pos[i];
        assert!(i < j);
        ans.push((i, j));
        a.swap(i, j);
        pos[a[i]] = i;
        pos[a[j]] = j;
    }
    println!("{}", ans.len());
    for (i, j) in ans {
        println!("{} {}", i + 1, j + 1);
    }
}
