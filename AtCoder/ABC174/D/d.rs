extern crate proconio;
use proconio::input;

fn main() {
    input! {
        n: i32,
        mut s: String,
    }
    let s = s.chars().collect::<Vec<_>>();
    let mut i = 0;
    let mut j = n - 1;
    let mut ans = 0;
    while i < j {
        while i < n && s[i as usize] == 'R' {
            i += 1;
        }
        while j >= 0 && s[j as usize] == 'W' {
            j -= 1;
        }
        if i >= j {
            break;
        }
        ans += 1;
        i += 1;
        j -= 1;
    }
    println!("{}", ans);
}
