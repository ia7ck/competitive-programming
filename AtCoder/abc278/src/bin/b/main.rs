use proconio::input;

fn f(h: u8, m: u8) -> bool {
    let mut hh = if h < 10 { [0, h] } else { [h / 10, h % 10] };
    let mut mm = if m < 10 { [0, m] } else { [m / 10, m % 10] };

    std::mem::swap(&mut hh[1], &mut mm[0]);

    let h = hh[0] * 10 + hh[1];
    let m = mm[0] * 10 + mm[1];
    h < 24 && m < 60
}

fn main() {
    input! {
        mut h: u8,
        mut m: u8,
    };

    loop {
        if f(h, m) {
            println!("{} {}", h, m);
            return;
        }
        m += 1;
        if m == 60 {
            h += 1;
            m = 0;
            if h == 24 {
                h = 0;
            }
        }
    }
}
