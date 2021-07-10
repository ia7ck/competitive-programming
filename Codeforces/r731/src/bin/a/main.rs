use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let q: usize = rd.get();
    for _ in 0..q {
        let xa: i32 = rd.get();
        let ya: i32 = rd.get();
        let xb: i32 = rd.get();
        let yb: i32 = rd.get();
        let xf: i32 = rd.get();
        let yf: i32 = rd.get();
        solve(xa, ya, xb, yb, xf, yf);
    }
}

fn solve(xa: i32, ya: i32, xb: i32, yb: i32, xf: i32, yf: i32) {
    let mut ans = (xa - xb).abs() + (ya - yb).abs();
    if xa == xb && xb == xf {
        if ya.min(yb) <= yf && yf <= ya.max(yb) {
            ans += 2;
        }
    } else if ya == yb && yb == yf {
        if xa.min(xb) <= xf && xf <= xa.max(xb) {
            ans += 2;
        }
    }
    println!("{}", ans);
}
