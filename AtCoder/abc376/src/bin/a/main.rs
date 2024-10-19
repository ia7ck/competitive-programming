use proconio::input;

fn main() {
    input! {
        n: usize,
        c: u32,
        t: [u32; n],
    };

    let mut last = Option::<u32>::None;
    let mut ans = 0;
    for t in t {
        last = match last {
            None => {
                ans += 1;
                Some(t)
            }
            Some(s) => {
                if t - s < c {
                    Some(s)
                } else {
                    ans += 1;
                    Some(t)
                }
            }
        }
    }
    println!("{}", ans);
}
