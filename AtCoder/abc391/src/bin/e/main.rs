use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: usize,
        a: Bytes,
    };

    let a = a.into_iter().map(|b| b - b'0').collect::<Vec<_>>();
    let last = {
        let mut a = a.clone();
        for _ in 0..n {
            a = apply(&a);
        }
        a[0]
    };

    let a = if last == 1 {
        a
    } else {
        a.into_iter().map(|x| 1 - x).collect()
    };

    // 1 -> 0
    let mut cost = vec![vec![]; n + 1];
    for i in 0..=n {
        cost[i] = vec![usize::MAX; 3_usize.pow(i as u32)];
    }
    for j in 0..a.len() {
        cost[n][j] = usize::from(a[j]);
    }
    for i in (0..n).rev() {
        for j in 0..3_usize.pow(i as u32) {
            let (p, q, r) = (j * 3, j * 3 + 1, j * 3 + 2);
            cost[i][j] = (cost[i + 1][p] + cost[i + 1][q])
                .min(cost[i + 1][q] + cost[i + 1][r])
                .min(cost[i + 1][r] + cost[i + 1][p]);
        }
    }

    println!("{}", cost[0][0]);
}

fn apply(a: &Vec<u8>) -> Vec<u8> {
    assert_eq!(a.len() % 3, 0);
    let mut b = vec![0; a.len() / 3];
    for i in 0..b.len() {
        // 多数決
        if a[i * 3] + a[i * 3 + 1] + a[i * 3 + 2] >= 2 {
            b[i] = 1;
        } else {
            b[i] = 0;
        }
    }
    b
}
