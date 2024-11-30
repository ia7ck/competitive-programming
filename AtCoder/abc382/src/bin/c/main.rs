use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    };

    let mut bi = Vec::new();
    for i in 0..m {
        bi.push((b[i], i));
    }

    bi.sort();
    let mut ans = vec![None; m];
    for (j, &a) in a.iter().enumerate() {
        while let Some(&(b, i)) = bi.last() {
            if b >= a {
                ans[i] = Some(j);
                bi.pop();
            } else {
                break;
            }
        }
    }

    for ans in ans {
        if let Some(ans) = ans {
            println!("{}", ans + 1);
        } else {
            println!("-1");
        }
    }
}
