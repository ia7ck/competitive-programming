use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: usize,
        s: Bytes,
        t: Bytes,
    };

    {
        let mut ss = s.clone();
        let mut tt = t.clone();
        ss.sort();
        tt.sort();
        if ss != tt {
            println!("-1");
            return;
        }
    }

    let mut j = n;
    let mut len = 0;
    for &b in s.iter().rev() {
        let mut found = false;
        for i in (0..j).rev() {
            if t[i] == b {
                found = true;
                j = i;
                break;
            }
        }
        if found == false {
            break;
        }
        len += 1;
    }

    let ans = n - len;
    println!("{}", ans);
}
