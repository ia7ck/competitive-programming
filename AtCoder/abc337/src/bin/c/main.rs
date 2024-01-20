use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
    };

    let mut behind = vec![0; n + 1];
    let mut x = 0;
    for i in 0..n {
        if a[i] == -1 {
            x = i + 1;
        } else {
            behind[a[i] as usize] = i + 1;
        }
    }
    assert_ne!(x, 0);
    let mut ans = vec![x];
    while behind[x] != 0 {
        x = behind[x];
        ans.push(x);
    }
    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
