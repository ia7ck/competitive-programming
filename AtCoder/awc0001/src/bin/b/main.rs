use proconio::input;

fn main() {
    input! {
        n: usize,
        l: i32,
        r: i32,
        p: [i32; n],
    };

    let mut ans = None;
    let mut p_max = -1;
    for i in 0..n {
        if l <= p[i] && p[i] <= r {
            if p_max < p[i] {
                ans = Some(i);
                p_max = p[i];
            }
        }
    }

    let Some(ans) = ans else {
        println!("-1");
        return;
    };

    println!("{}", ans + 1);
}
