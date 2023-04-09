use proconio::input;

fn main() {
    // f(3, 14);

    input! {
        n: usize,
        l: u64,
        r: u64,
        a: [u64; n],
    };

    // 周期 l+r
    // 0 が l 個
    // 1 が l 個
    // 2 が l 個
    // (r/l) が l 個
    // (r/l+1) が r%l 個

    let mut g = 0;
    for x in a {
        let x = x % (l + r);
        let h = x / l;
        g ^= h;
    }

    if g != 0 {
        println!("First");
    } else {
        println!("Second");
    }
}

#[allow(unused)]
fn f(l: usize, r: usize) {
    let mut grundy = vec![0; 100];
    for s in 1..100 {
        let mut seen = vec![false; 100];
        for x in l..=r {
            if s >= x {
                seen[grundy[s - x]] = true;
            }
        }
        for v in 0..100 {
            if seen[v] == false {
                grundy[s] = v;
                break;
            }
        }
    }

    for v in 0..100 {
        eprintln!("grundy[{}] = {}", v, grundy[v]);
    }
}
