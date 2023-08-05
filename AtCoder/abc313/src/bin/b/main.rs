use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    };

    let mut strongest = Vec::new();
    for x in 1..=n {
        let mut ok = true;
        for &(_, b) in &ab {
            if b == x {
                ok = false;
            }
        }
        if ok {
            strongest.push(x);
        }
    }

    if strongest.len() == 1 {
        println!("{}", strongest[0]);
    } else {
        println!("-1");
    }
}
