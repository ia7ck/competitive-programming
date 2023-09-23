use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: Bytes,
    };

    let mut ok = true;
    for w in n.windows(2) {
        if w[0] <= w[1] {
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
