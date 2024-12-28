use proconio::input;

fn main() {
    input! {
        mut xs: [u8; 4],
    };

    xs.sort();
    let is22 = xs[0] == xs[1] && xs[1] != xs[2] && xs[2] == xs[3];
    let is13 = xs[0] != xs[1] && xs[1] == xs[2] && xs[2] == xs[3];
    let is31 = xs[0] == xs[1] && xs[1] == xs[2] && xs[2] != xs[3];
    if is22 || is13 || is31 {
        println!("Yes");
    } else {
        println!("No");
    }
}
