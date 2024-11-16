use proconio::{input, marker::Chars};
use run_length::RunLength;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    };

    let a = RunLength::new(&s).collect::<Vec<_>>();
    let i = {
        let mut one = 0;
        let mut pos = None;
        for i in 0..n {
            let (c, _) = a[i];
            if c == &'1' {
                one += 1;
            }
            if one == k {
                pos = Some(i);
                break;
            }
        }
        pos.unwrap()
    };

    assert!(i >= 1);

    let mut a = a;
    a.swap(i - 1, i);
    let mut ans = String::new();
    for (&c, len) in a {
        for _ in 0..len {
            ans.push(c);
        }
    }
    println!("{}", ans);
}
