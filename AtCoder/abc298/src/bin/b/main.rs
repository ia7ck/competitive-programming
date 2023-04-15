use proconio::input;

fn check(a: &Vec<Vec<u8>>, b: &Vec<Vec<u8>>) -> bool {
    let mut ok = true;
    for i in 0..a.len() {
        for j in 0..a[i].len() {
            if a[i][j] == 1 {
                if b[i][j] == 0 {
                    ok = false;
                }
            }
        }
    }
    ok
}

fn main() {
    input! {
        n: usize,
        mut a: [[u8; n]; n],
        b: [[u8; n]; n],
    };

    for _ in 0..4 {
        let mut c = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                c[i][j] = a[n - j - 1][i];
            }
        }
        a = c;
        if check(&a, &b) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
