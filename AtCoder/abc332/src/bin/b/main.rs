use proconio::input;

fn main() {
    input! {
        k: usize,
        g: i32,
        m: i32,
    };

    let mut gg = 0;
    let mut mm = 0;
    for _ in 0..k {
        if gg == g {
            gg = 0;
        } else if mm == 0 {
            mm = m;
        } else {
            if gg + mm <= g {
                gg += mm;
                mm = 0;
            } else {
                mm -= g - gg;
                gg = g;
            }
        }
    }
    println!("{} {}", gg, mm);
}
