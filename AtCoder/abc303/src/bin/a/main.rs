use proconio::{input, marker::Chars};

fn similar(x: char, y: char) -> bool {
    x == y
        || (x == '1' && y == 'l')
        || (x == 'l' && y == '1')
        || (x == '0' && y == 'o')
        || (x == 'o' && y == '0')
}

fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    };

    let mut ok = true;
    for i in 0..n {
        if similar(s[i], t[i]) == false {
            ok = false;
            break;
        }
    }

    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
