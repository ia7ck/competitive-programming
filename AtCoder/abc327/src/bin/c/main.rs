use proconio::input;

const N: usize = 9;

fn check(a: &[u8]) -> bool {
    let mut a = a.to_vec();
    a.sort();
    a == (1..=(N as u8)).collect::<Vec<_>>()
}

fn main() {
    input! {
        a: [[u8; N]; N],
    };

    let mut b = Vec::new();
    for i in 0..N {
        b.push(a[i].clone());
    }
    for j in 0..N {
        let mut c = Vec::new();
        for i in 0..N {
            c.push(a[i][j]);
        }
        b.push(c);
    }
    for i in (0..N).step_by(3) {
        for j in (0..N).step_by(3) {
            let mut c = Vec::new();
            for k in 0..3 {
                for l in 0..3 {
                    c.push(a[i + k][j + l]);
                }
            }
            b.push(c);
        }
    }
    let ok = b.iter().all(|c| check(c));
    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
