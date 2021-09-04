use std::collections::HashMap;

use mod_int::ModInt998244353;
use procon_reader::ProconReader;
type Mint = ModInt998244353;

fn solve(
    l: usize,
    r: usize,
    ok: &Vec<Vec<bool>>,
    cmb: &Vec<Vec<Mint>>,
    memo: &mut HashMap<(usize, usize), Mint>,
) -> Mint {
    if l == r {
        return Mint::new(1);
    }
    if let Some(&ans) = memo.get(&(l, r)) {
        return ans;
    }
    let mut ans = Mint::new(0);
    // make pair (l, i)
    for i in ((l + 1)..r).step_by(2) {
        if ok[l][i] {
            // [l+1, i), [i+1, r)
            let l_val = solve(l + 1, i, ok, cmb, memo);
            let r_val = solve(i + 1, r, ok, cmb, memo);
            ans += l_val * r_val * cmb[(r - l) / 2][(r - (i + 1)) / 2];
        }
    }
    memo.insert((l, r), ans);
    ans
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let m: usize = rd.get();
    let mut ok = vec![vec![false; n * 2]; n * 2];
    for _ in 0..m {
        let a: usize = rd.get();
        let b: usize = rd.get();
        ok[a - 1][b - 1] = true;
        ok[b - 1][a - 1] = true;
    }

    let mut cmb = vec![vec![Mint::new(0); n + 1]; n + 1];
    cmb[0][0] = Mint::new(1);
    for i in 1..=n {
        cmb[i][0] = Mint::new(1);
        for j in 1..=i {
            cmb[i][j] = cmb[i - 1][j - 1] + cmb[i - 1][j];
        }
    }
    let mut memo = HashMap::new();
    let ans = solve(0, n * 2, &ok, &cmb, &mut memo);
    println!("{}", ans.val());
}
