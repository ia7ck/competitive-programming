use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };

    let mut ans = 0;
    if let Some(i) = a[..k].iter().position(|&a| a == 0) {
        let unlock = a[i..k].iter().sum::<usize>();
        let lock = k - i;
        ans += unlock + lock;
    }
    if let Some(i) = a[k..].iter().rev().position(|&a| a == 0) {
        let unlock = a[k..(n - i)].iter().sum::<usize>();
        let lock = (n - i) - k;
        ans += unlock + lock;
    }

    println!("{}", ans);
}
