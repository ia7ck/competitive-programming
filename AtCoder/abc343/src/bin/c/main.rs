use proconio::input;

fn main() {
    input! {
        n: u64,
    };

    let mut ans = 1;
    for x in 1.. {
        let k = x * x * x;
        if k > n {
            break;
        }
        if palindrome(k) {
            ans = k;
        }
    }

    println!("{}", ans);
}

fn palindrome(k: u64) -> bool {
    let s = k.to_string().chars().collect::<Vec<_>>();
    let mut t = s.clone();
    t.reverse();
    s == t
}
