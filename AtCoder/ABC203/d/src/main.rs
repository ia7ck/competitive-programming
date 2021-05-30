use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let k: usize = rd.get();
    let a: Vec<Vec<u64>> = (0..n).map(|_| rd.get_vec(n)).collect();

    let judge = |x: u64| -> bool {
        let mut b = vec![vec![0u64; n + 1]; n + 1];
        for i in 0..n {
            for j in 0..n {
                if a[i][j] > x {
                    b[i + 1][j + 1] = 1;
                }
            }
        }
        for i in 0..=n {
            for j in 0..n {
                b[i][j + 1] += b[i][j];
            }
        }
        for j in 0..=n {
            for i in 0..n {
                b[i + 1][j] += b[i][j];
            }
        }
        for i in k..=n {
            for j in k..=n {
                let s = b[i][j] - b[i][j - k] - b[i - k][j] + b[i - k][j - k];
                if s < (k * k / 2 + 1) as u64 {
                    return true;
                }
            }
        }
        false
    };

    let mut mn = a[0][0];
    let mut mx = a[0][0];
    for i in 0..n {
        for j in 0..n {
            mn = mn.min(a[i][j]);
            mx = mx.max(a[i][j]);
        }
    }
    if judge(mn) {
        println!("{}", mn);
        return;
    }
    let mut ng = mn;
    let mut ok = mx;
    while ok - ng > 1 {
        let m = (ng + ok) / 2;
        if judge(m) {
            ok = m;
        } else {
            ng = m;
        }
    }
    println!("{}", ok);
}
