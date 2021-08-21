use join::Join;
use least_prime_factors::least_prime_factors;
use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let m: usize = rd.get();
    let a: Vec<usize> = rd.get_vec(n);

    let l = 1_000_00;
    let lpf = least_prime_factors(l + 1);
    let mut ok = vec![true; l + 1];
    for mut a in a {
        while a > 1 {
            let p = lpf[a];
            ok[p] = false;
            a /= p;
        }
    }
    for i in 1..=m {
        if !ok[i] {
            for j in (i..=m).step_by(i) {
                ok[j] = false;
            }
        }
    }
    let mut ans = Vec::new();
    for i in 1..=m {
        if ok[i] {
            ans.push(i);
        }
    }
    println!("{}", ans.len());
    println!("{}", ans.iter().join("\n"));
}
