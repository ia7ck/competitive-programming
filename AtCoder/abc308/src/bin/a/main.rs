use proconio::input;

fn main() {
    input! {
        s: [u32; 8],
    };

    let mut ok = true;
    for i in 1..8 {
        ok &= s[i - 1] <= s[i];
    }
    for i in 0..8 {
        ok &= 100 <= s[i] && s[i] <= 675;
    }
    for i in 0..8 {
        ok &= s[i] % 25 == 0;
    }
    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
