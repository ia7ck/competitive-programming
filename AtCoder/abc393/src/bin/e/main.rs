use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };

    let mut count = vec![0; 1_000_000 + 1];
    for &a in &a {
        count[a] += 1;
    }
    // mul[x] := #{ y ; y % x == 0 }
    let mut mul = vec![0; 1_000_000 + 1];
    for x in 1..=1_000_000 {
        for y in (x..=1_000_000).step_by(x) {
            mul[x] += count[y];
        }
    }
    // g[x] := max { d ; x % d == 0 && mul[d] >= k }
    let mut g = vec![0; 1_000_000 + 1];
    for d in 1..=1_000_000 {
        for x in (d..=1_000_000).step_by(d) {
            if mul[d] >= k {
                g[x] = g[x].max(d);
            }
        }
    }

    for a in a {
        println!("{}", g[a]);
    }
}
