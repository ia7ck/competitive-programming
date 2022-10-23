use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {
        n: usize,
        a: i32,
        wxv: [(u32, i32, i32); n],
    };

    let mut ans = 0;
    for i in 0..n {
        let (_, x, v) = wxv[i];
        let mut events = Vec::new();
        #[derive(Debug, PartialEq)]
        enum E {
            In(f64, usize),
            Out(f64, usize),
        }
        impl PartialOrd for E {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                match (self, other) {
                    (E::In(t, _), E::In(tt, _)) => t.partial_cmp(tt),
                    (E::Out(t, _), E::Out(tt, _)) => t.partial_cmp(tt),
                    // 有理数で比較したほうがよい？
                    (E::In(t, _), E::Out(tt, _)) => t
                        .partial_cmp(tt)
                        .map(|ord| ord.then_with(|| Ordering::Less)),
                    (E::Out(t, _), E::In(tt, _)) => t
                        .partial_cmp(tt)
                        .map(|ord| ord.then_with(|| Ordering::Greater)),
                }
            }
        }
        for j in 0..n {
            let (_, xx, vv) = wxv[j];
            if vv > v {
                events.push(E::In((x - xx) as f64 / (vv - v) as f64, j));
                events.push(E::Out((x + a - xx) as f64 / (vv - v) as f64, j));
            } else if vv == v {
                if x <= xx && xx <= x + a {
                    events.push(E::In(0.0, j));
                }
            } else {
                // vv < v
                events.push(E::In((xx - (x + a)) as f64 / (v - vv) as f64, j));
                events.push(E::Out((xx - x) as f64 / (v - vv) as f64, j));
            }
        }
        events.sort_by(|e1, e2| e1.partial_cmp(e2).unwrap());
        let mut sum = 0;
        for e in events {
            match e {
                E::In(t, j) => {
                    let (ww, _, _) = wxv[j];
                    sum += ww;
                    if t >= 0.0 {
                        ans = ans.max(sum);
                    }
                }
                E::Out(t, j) => {
                    let (ww, _, _) = wxv[j];
                    assert!(sum >= ww);
                    sum -= ww;
                    if t >= 0.0 {
                        ans = ans.max(sum);
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
