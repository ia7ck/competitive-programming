use proconio::input;

fn main() {
    input! {
        q: usize,
    };

    let mut nexts = vec![Option::<usize>::None; q + 1];
    for i in 1..=q {
        input! {
            op: u8,
        };
        if op == 1 {
            input! {
                x: usize,
            };

            match nexts[x] {
                Some(n) => {
                    nexts[x] = Some(i);
                    nexts[i] = Some(n);
                }
                None => {
                    nexts[x] = Some(i);
                }
            }
        } else {
            input! {
                x: usize,
                y: usize,
            };

            let mut xs = vec![x];
            let mut ys = vec![y];
            while xs.last() != Some(&y) && ys.last() != Some(&x) {
                let lx = xs.last().copied().unwrap();
                let ly = ys.last().copied().unwrap();
                match nexts[lx] {
                    Some(n) => {
                        xs.push(n);
                    }
                    None => {
                        xs.push(lx);
                    }
                }
                match nexts[ly] {
                    Some(n) => {
                        ys.push(n);
                    }
                    None => {
                        ys.push(ly);
                    }
                }
            }
            let mut ans = 0;
            if xs.last() == Some(&y) {
                let n = xs.len();
                for i in 1..(n - 1) {
                    ans += xs[i];
                    nexts[xs[i]] = None;
                }
                nexts[xs[0]] = Some(xs[n - 1]);
            } else {
                assert_eq!(ys.last(), Some(&x));
                let n = ys.len();
                for i in 1..(n - 1) {
                    ans += ys[i];
                    nexts[ys[i]] = None;
                }
                nexts[ys[0]] = Some(ys[n - 1]);
            }
            println!("{}", ans);
        }
    }
}
