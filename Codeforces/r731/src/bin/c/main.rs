use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let t: usize = rd.get();
    for _ in 0..t {
        let k: usize = rd.get();
        let n: usize = rd.get();
        let m: usize = rd.get();
        let a: Vec<usize> = rd.get_vec(n);
        let b: Vec<usize> = rd.get_vec(m);
        solve(k, n, m, a, b);
    }
}

fn solve(k: usize, n: usize, m: usize, a: Vec<usize>, b: Vec<usize>) {
    use std::collections::VecDeque;
    let mut a: VecDeque<usize> = a.into_iter().collect();
    let mut b: VecDeque<usize> = b.into_iter().collect();
    let mut k = k;
    let mut ans = Vec::new();
    while a.len() >= 1 || b.len() >= 1 {
        while a.len() >= 1 && a[0] == 0 {
            ans.push(a.pop_front().unwrap());
            k += 1;
        }
        while b.len() >= 1 && b[0] == 0 {
            ans.push(b.pop_front().unwrap());
            k += 1;
        }
        if a.len() >= 1 && a[0] <= k {
            ans.push(a.pop_front().unwrap());
        } else if b.len() >= 1 && b[0] <= k {
            ans.push(b.pop_front().unwrap());
        } else {
            break;
        }
    }
    if ans.len() != n + m {
        println!("-1");return;
    }
    print!("{}", ans[0]);
    for a in &ans[1..] {
        print!(" {}", a);
    }
    println!();
}
