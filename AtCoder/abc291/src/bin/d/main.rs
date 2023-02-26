use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(u32, u32); n],
    };

    const M: u64 = 998244353;

    macro_rules! add {
        ($a: expr, $b: expr) => {
            $a = ($a + $b) % M;
        };
    }
    
    let mut dp = [1, 1];
    for i in 1..n {
        let (a, b) = ab[i - 1];
        let (aa, bb) = ab[i];
        let mut next = [0, 0];
        if a != aa {
            add!(next[0], dp[0]);
        }
        if a != bb {
            add!(next[1], dp[0]);
        }
        if b != aa {
            add!(next[0], dp[1]);
        }
        if b != bb {
            add!(next[1], dp[1]);
        }
        dp = next;
    }

    let ans = (dp[0] + dp[1]) % M;
    println!("{}", ans);
}
