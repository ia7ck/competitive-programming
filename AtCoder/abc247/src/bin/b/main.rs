use proconio::input;

fn main() {
    input! {
        n: usize,
        st: [(String, String); n],
    };

    for i in 0..n {
        let (si, ti) = &st[i];
        let mut ok_s = true;
        let mut ok_t = true;
        for j in 0..n {
            if i == j {
                continue;
            }
            let (sj, tj) = &st[j];
            if si == sj || si == tj {
                ok_s = false;
            }
            if ti == sj || ti == tj {
                ok_t = false;
            }
        }
        if ok_s || ok_t {
            // ok
        } else {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
