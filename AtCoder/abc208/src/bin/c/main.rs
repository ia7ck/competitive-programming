use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let k: usize = rd.get();
    let a: Vec<usize> = rd.get_vec(n);

    let mut ai = Vec::new();
    for i in 0..n {
        ai.push((a[i], i));
    }
    ai.sort();
    let mut ans = vec![k / n; n];
    for &(_, i) in ai.iter().take(k % n) {
        ans[i] += 1;
    }
    for ans in ans {
        println!("{}", ans);
    }
}
