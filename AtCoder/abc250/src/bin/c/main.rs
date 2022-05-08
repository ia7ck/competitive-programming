use join::Join;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        x: [usize; q],
    };

    let x: Vec<usize> = x.into_iter().map(|x| x - 1).collect();
    let mut a: Vec<usize> = (0..n).collect();
    let mut pos: Vec<usize> = a.clone();
    for x in x {
        let p = pos[x];
        let q = if p + 1 == n { p - 1 } else { p + 1 };
        let y = a[q];
        a.swap(p, q);
        pos.swap(x, y);
    }

    println!("{}", a.iter().map(|&a| a + 1).join(" "));
}
