use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    };

    let mut down = vec![Option::<usize>::None; n + 1];
    let mut top = vec![Option::<usize>::None; n + 1];
    for x in 1..=n {
        top[x] = Some(x);
    }
    for _ in 0..q {
        input! {
            f: usize,
            t: usize,
            x: usize,
        };
        let top_of_f = top[f].unwrap(); // maybe x
        if let Some(d) = down[x] {
            top[f] = Some(d);
        } else {
            top[f] = None;
        }
        if let Some(y) = top[t] {
            down[x] = Some(y);
            top[t] = Some(top_of_f);
        } else {
            down[x] = None;
            top[t] = Some(top_of_f);
        }
    }

    let mut ans = vec![0; n + 1];
    for i in 1..=n {
        let mut x = top[i];
        while let Some(y) = x {
            ans[y] = i;
            x = down[y];
        }
    }
    for i in 1..=n {
        println!("{}", ans[i]);
    }
}
