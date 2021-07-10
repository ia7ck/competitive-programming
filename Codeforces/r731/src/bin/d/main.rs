use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let t: usize = rd.get();
    for _ in 0..t {
        let n: usize = rd.get();
        let x: Vec<u32> = rd.get_vec(n);
        solve(n, x);
    }
}

fn solve(n: usize, x: Vec<u32>) {
    let mut ans = vec![0_u32];
    for i in 1..n {
        let p = x[i - 1] ^ ans[ans.len() - 1];
        let mut y = 0;
        for j in 0..30 {
            if p >> j & 1 == 1 {
                if x[i] >> j & 1 == 0 {
                    y ^= 1 << j;
                }
            }
        }
        ans.push(y);
    }
    print!("{}", ans[0]);
    for a in &ans[1..] {
        print!(" {}", a);
    }
    println!();
}
