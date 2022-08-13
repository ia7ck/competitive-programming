use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut a: Vec<u8> = s
        .into_iter()
        .map(|ch| match ch {
            'a' => 0,
            't' => 1,
            'c' => 2,
            'o' => 3,
            'd' => 4,
            'e' => 5,
            'r' => 6,
            _ => unreachable!(),
        })
        .collect();

    let mut ans = 0;
    for i in 0..a.len() {
        let min = a[i..].iter().copied().min().unwrap();
        let j = a.iter().position(|&ch| ch == min).unwrap();
        if i == j {
            continue;
        }
        assert!(i < j);
        for k in (i..j).rev() {
            ans += 1;
            a.swap(k, k + 1);
        }
    }
    println!("{}", ans);
}
