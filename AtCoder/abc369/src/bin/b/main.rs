use proconio::input;

fn main() {
    input! {
        n: usize,
        press: [(u32, char); n],
    };

    let mut ans = u32::MAX;
    for start_l in 1..=100_u32 {
        for start_r in 1..=100_u32 {
            let mut cost = 0;
            let (mut l, mut r) = (start_l, start_r);
            for &(p, c) in &press {
                match c {
                    'L' => {
                        cost += l.abs_diff(p);
                        l = p;
                    }
                    'R' => {
                        cost += r.abs_diff(p);
                        r = p;
                    },
                    _ => unreachable!()
                }
            }
            ans = ans.min(cost);
        }
    }
    println!("{}", ans);
}
