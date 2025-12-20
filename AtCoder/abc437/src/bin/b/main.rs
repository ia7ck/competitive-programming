use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        a: [[u32; w]; h],
        b: [u32; n],
    };

    let mut ans = 0;
    for i in 0..h {
        let hit = b.iter().filter(|&&b| a[i].contains(&b)).count();
        ans = ans.max(hit);
    }
    println!("{}", ans);
}
