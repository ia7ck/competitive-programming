use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: usize,
        s: Bytes,
    };

    let ok = if s[0] == b'M' {
        let mut t = Vec::new();
        for i in 0..n {
            if i % 2 == 0 {
                t.push(b'M');
            } else {
                t.push(b'F');
            }
        }
        s == t
    } else {
        let mut t = Vec::new();
        for i in 0..n {
            if i % 2 == 1 {
                t.push(b'M');
            } else {
                t.push(b'F');
            }
        }
        s == t
    };

    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
