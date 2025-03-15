use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut t = s.clone();
    loop {
        let mut change = false;
        for i in 0..t.len() {
            if i % 2 == 0 && t[i] == 'o' {
                change = true;
                t.insert(i, 'i');
                break;
            } else if i % 2 == 1 && t[i] == 'i' {
                change = true;
                t.insert(i, 'o');
                break;
            }
        }
        if !change {
            break;
        }
    }

    if t.len() % 2 == 1 {
        t.push('o');
    }

    let ans = t.len() - s.len();
    println!("{}", ans);
}
