use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: usize,
        s: Bytes,
    };

    let mut t = 0;
    for i in 0..n {
        let d = u64::from(s[i] - b'0');
        t += d * (i + 1) as u64;
    }
    let mut carry = 0;
    let mut ans = Vec::new();
    for i in (0..n).rev() {
        ans.push((t + carry) % 10);
        carry = (t + carry) / 10;
        let d = u64::from(s[i] - b'0');
        t -= d * (i + 1) as u64;
    }
    if carry > 0 {
        ans.push(carry);
    }
    ans.reverse();
    for ans in ans {
        print!("{}", ans);
    }
    println!();
}
