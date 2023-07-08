use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(u64, u64); n],
    };

    let mut abi = ab.into_iter().enumerate().collect::<Vec<_>>();
    abi.sort_by(|(i, (a, b)), (ii, (aa, bb))| {
        // a / (a + b) <=> aa / (aa + bb)
        (aa * (a + b)).cmp(&(a * (aa + bb))).then_with(|| i.cmp(ii))
    });
    for i in 0..n {
        let (ans, _) = abi[i];
        print!("{}", ans + 1);
        if i + 1 < n {
            print!(" ");
        } else {
            print!("\n");
        }
    }
}
