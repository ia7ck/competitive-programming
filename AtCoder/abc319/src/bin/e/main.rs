use proconio::input;

fn main() {
    input! {
        n: usize,
        x: u64,
        y: u64,
        pt: [(u64, u64); n - 1],
        q: usize,
        qs: [u64; q],
    };

    let l = 840;
    let mut cost = vec![0; l as usize];
    for s in 0..l {
        let mut h = s;
        h += x;
        for &(p, t) in &pt {
            h = (h + p - 1) / p * p; // ceil(h/p)
            h += t;
        }
        h += y;
        cost[s as usize] = h - s;
    }


    for q in qs {
        let ans = q + cost[(q % l) as usize];
        println!("{}", ans);
    }
}
