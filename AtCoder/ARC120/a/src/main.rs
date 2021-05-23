use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let a: Vec<u64> = rd.get_vec(n);

    // a[0] + max
    // a[0] * 2 + a[1] + max * 2
    // a[0] * 3 + a[1] * 2 + a[2] + max * 3

    let mut max = 0;
    let mut prefix_sum = 0;
    let mut ans = 0;
    for i in 0..n {
        max = max.max(a[i]);
        prefix_sum += a[i];
        ans += prefix_sum;
        // println!("max = {}", max);
        // println!("prefix = {}", prefix_sum);
        println!("{}", ans + max * (i + 1) as u64);
    }
}
