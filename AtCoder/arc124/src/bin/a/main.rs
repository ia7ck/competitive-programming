use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let k: usize = rd.get();
    let cp: Vec<(char, usize)> = (0..k)
        .map(|_| {
            let c: char = rd.get();
            let p: usize = rd.get();
            (c, p)
        })
        .collect();

    if k > n {
        println!("0");
        return;
    }
    let mut lr = vec!['?'; n + 1];
    let mut ways = vec![0; n + 1];
    for (c, p) in cp {
        lr[p] = c;
        ways[p] = 1;
    }
    for i in 1..=n {
        if lr[i] == 'L' {
            for j in (i + 1)..=n {
                if lr[j] == '?' {
                    ways[j] += 1;
                }
            }
        } else if lr[i] == 'R' {
            for j in (1..=(i - 1)).rev() {
                if lr[j] == '?' {
                    ways[j] += 1;
                }
            }
        }
    }
    let mo: u64 = 998244353;
    let mut ans = 1;
    for i in 1..=n {
        ans = ans * ways[i] % mo;
    }
    println!("{}", ans);
}
