use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[u8; w]; h],
    };

    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 0 {
                print!(".");
            } else {
                let ch = (b'A' + a[i][j] - 1) as char;
                print!("{}", ch);
            }
        }
        println!();
    }
}
