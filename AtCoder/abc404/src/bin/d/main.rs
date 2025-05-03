use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        c: [u64; n],
        a: [[Usize1]; m],
    };

    let mut zoo_animals = vec![vec![]; n];
    for animal in 0..m {
        for &zoo in &a[animal] {
            zoo_animals[zoo].push(animal);
        }
    }
    let mut ans = u64::MAX;
    for bits in 0..(1 << n) {
        for sub in 0..(1 << n) {
            if bits & sub == sub {
                let mut seen = vec![0; m];
                let mut cost = 0;
                for i in 0..n {
                    if bits >> i & 1 == 1 {
                        for &a in &zoo_animals[i] {
                            seen[a] += 1;
                        }
                        cost += c[i];
                    }
                    if sub >> i & 1 == 1 {
                        for &a in &zoo_animals[i] {
                            seen[a] += 1;
                        }
                        cost += c[i];
                    }
                }
                if seen.iter().all(|&s| s >= 2) {
                    ans = ans.min(cost);
                }
            }
        }
    }

    println!("{}", ans);
}
