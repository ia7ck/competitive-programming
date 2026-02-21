use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: [[usize]; n],
    };

    let mut select = vec![false; m + 1];
    for x in x {
        let ans = x
            .iter()
            .find(|&&x| select[x] == false)
            .copied()
            .unwrap_or(0);
        println!("{}", ans);

        if ans > 0 {
            select[ans] = true;
        }
    }
}
