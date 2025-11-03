use ac_library::{convolution, ModInt998244353};
use proconio::input;

type Mint = ModInt998244353;

fn main() {
    input! {
        d: usize,
        n: usize,
    };

    let mut f = vec![Mint::new(0); 7];
    for i in [1, 3, 4, 6] {
        f[i] = 1.into();
    }

    let mut fs = vec![f.clone(); d];
    while fs.len() > 1 {
        let mut new_fs = Vec::with_capacity(fs.len() / 2);
        while let Some(f) = fs.pop() {
            if let Some(g) = fs.pop() {
                let h = convolution::convolution(&f, &g);
                new_fs.push(h);
            } else {
                new_fs.push(f);
            }
        }
        fs = new_fs;
    }
    println!("{}", fs[0][n].val());
}
