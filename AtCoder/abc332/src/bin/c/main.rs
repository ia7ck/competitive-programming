use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
    };

    let check = |buy: usize| -> bool {
        let mut normal = m;
        let mut logo = buy;
        let mut fail = false;
        for &ch in &s {
            match ch {
                '0' => {
                    normal = m;
                    logo = buy;
                }
                '1' => {
                    if normal >= 1 {
                        normal -= 1;
                    } else if logo >= 1 {
                        logo -= 1;
                    } else {
                        fail = true;
                        break;
                    }
                }
                '2' => {
                    if logo >= 1 {
                        logo -= 1;
                    } else {
                        fail = true;
                        break;
                    }
                }
                _ => unreachable!()
            }
        }
        fail == false
    };

    if check(0) {
        println!("0");
        return;
    }

    let mut ng = 0;
    let mut ok = n;
    while ok - ng > 1 {
        let mid = (ok + ng) / 2;
        if check(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}
