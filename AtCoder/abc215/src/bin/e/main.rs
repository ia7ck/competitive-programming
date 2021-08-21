use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let s: Vec<char> = rd.get_chars();

    macro_rules! add {
        ($a: expr, $b: expr) => {
            $a = ($a + $b) % 998244353;
        };
    }

    let mut dp = vec![vec![0; 10]; 1 << 10];
    for i in 0..n {
        let mut nxt = dp.clone();
        let si = s[i] as usize - 'A' as usize;
        // println!("si = {}", si);
        add!(nxt[1 << si][si], 1);
        for bits in 1..(1 << 10) {
            for c in 0..10 {
                if bits >> si & 1 == 1 {
                    if si == c {
                        add!(nxt[bits][si], dp[bits][c]);
                    }
                } else {
                    add!(nxt[bits ^ (1 << si)][si], dp[bits][c]);
                }
            }
        }
        dp = nxt;
    }
    let mut ans = 0;
    for bits in 1..(1 << 10) {
        for c in 0..10 {
            add!(ans, dp[bits][c]);
        }
    }
    println!("{}", ans);
}
