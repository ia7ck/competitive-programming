use proconio::{
    input,
    marker::{Bytes, Usize1},
};

fn main() {
    input! {
        n: usize,
        ab: [(usize, Usize1); n],
        m: usize,
        s: [Bytes; m],
    };

    let mut c = vec![vec![vec![0; 26]; 10]; 11];
    for s in &s {
        for j in 0..s.len() {
            let k = usize::from(s[j] - b'a');
            c[s.len()][j][k] += 1;
        }
    }

    for s in s {
        if s.len() != n {
            println!("No");
            continue;
        }
        let mut ok = true;
        for (i, &(a, b)) in ab.iter().enumerate() {
            let k = usize::from(s[i] - b'a');
            ok &= c[a][b][k] >= 1;
        }
        if ok {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
