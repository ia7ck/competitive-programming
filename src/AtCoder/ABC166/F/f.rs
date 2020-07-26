extern crate proconio;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: i64,
        mut b: i64,
        mut c: i64,
        s: [String; n],
    };

    let mut ans = vec![];
    for i in 0..n {
        let t = &s[i];
        if t == "AB" {
            if a < b || (a == b && i + 1 < n && s[i + 1] == "AC") {
                a += 1;
                b -= 1;
                ans.push("A");
            } else {
                a -= 1;
                b += 1;
                ans.push("B");
            }
        } else if t == "BC" {
            if b < c || (b == c && i + 1 < n && s[i + 1] == "AB") {
                b += 1;
                c -= 1;
                ans.push("B");
            } else {
                b -= 1;
                c += 1;
                ans.push("C");
            }
        } else {
            if c < a || (c == a && i + 1 < n && s[i + 1] == "BC") {
                c += 1;
                a -= 1;
                ans.push("C");
            } else {
                c -= 1;
                a += 1;
                ans.push("A");
            }
        }
        if a < 0 || b < 0 || c < 0 {
            println!("No");
            return;
        }
    }
    println!("Yes");
    println!("{}", ans.join("\n"));
}
