use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, char); m],
    };

    let mut yes = vec![false; n];
    for (a, b) in ab {
        if b == 'M' && !yes[a] {
            println!("Yes");
            yes[a] = true;
        } else {
            println!("No");
        }
    }
}
