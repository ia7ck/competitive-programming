use proconio::input;

use next_permutation::NextPermutation;

fn main() {
    input! {
        c: [[u8; 3]; 3],
    };

    let mut ord = (0..9).collect::<Vec<_>>();
    let mut cnt = 0;
    loop {
        let mut ok = true;
        let mut a = [[None; 3]; 3];
        for &t in &ord {
            let (i, j) = (t / 3, t % 3);
            let tate = (0..3).filter_map(|k| a[k][j]).collect::<Vec<_>>();
            if tate.len() == 2 && tate[0] == tate[1] {
                ok = false;
                break;
            }
            let yoko = (0..3).filter_map(|k| a[i][k]).collect::<Vec<_>>();
            if yoko.len() == 2 && yoko[0] == yoko[1] {
                ok = false;
                break;
            }
            if [(0, 0), (1, 1), (2, 2)].contains(&(i, j)) {
                let naname = [(0, 0), (1, 1), (2, 2)]
                    .iter()
                    .filter_map(|&(k, l)| a[k][l])
                    .collect::<Vec<_>>();
                if naname.len() == 2 && naname[0] == naname[1] {
                    ok = false;
                    break;
                }
            }
            if [(0, 2), (1, 1), (2, 0)].contains(&(i, j)) {
                let naname = [(0, 2), (1, 1), (2, 0)]
                    .iter()
                    .filter_map(|&(k, l)| a[k][l])
                    .collect::<Vec<_>>();
                if naname.len() == 2 && naname[0] == naname[1] {
                    ok = false;
                    break;
                }
            }
            a[i][j] = Some(c[i][j]);
        }
        if ok {
            cnt += 1;
        }
        if ord.next_permutation() == false {
            break;
        }
    }

    let total = 9 * 8 * 7 * 6 * 5 * 4 * 3 * 2 * 1;
    let ans = cnt as f64 / total as f64;
    println!("{}", ans);
}
