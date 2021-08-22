use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let m: usize = rd.get();
    let s: Vec<u8> = rd.get_vec(n);
    let t: Vec<u8> = rd.get_vec(m);

    if let Some(mut ans) = solve(n, m, &s, &t) {
        let mut ss = vec![s[0]];
        for i in (1..n).rev() {
            ss.push(s[i]);
        }
        if let Some(tmp) = solve(n, m, &ss, &t) {
            ans = ans.min(tmp);
        }
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

fn solve(n: usize, m: usize, s: &[u8], t: &[u8]) -> Option<usize> {
    if n == 1 {
        return if t.iter().all(|&x| x == s[0]) {
            Some(m)
        } else {
            None
        };
    }
    let mut s = s.clone().to_vec();
    let mut hit = false;
    let mut res = 0;
    for &x in t {
        if hit {
            if x == s[0] {
                res += 1;
            } else {
                if x == s[1] {
                    s.rotate_left(1);
                    res += 1 + 1;
                } else if x == s[n - 1] {
                    s.rotate_right(1);
                    res += 1 + 1;
                } else {
                    unreachable!();
                }
            }
        } else {
            if x == s[0] {
                res += 1;
            } else {
                let p = s.iter().position(|&y| x == y)?;
                // eprintln!("{:?}", s);
                // eprintln!("p = {}", p);
                s.rotate_left(p);
                // eprintln!("{:?}", s);
                res += p + 1;
                hit = true;
            }
        }
    }
    Some(res)
}
