use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(u32, u32); n],
    };

    let mut xs = 0;
    let mut ys = 0;
    for (x, y) in xy {
        xs += x;
        ys += y;
    }
    if xs > ys {
        println!("Takahashi");
    } else if xs < ys {
        println!("Aoki");
    } else {
        println!("Draw");
    }
}
