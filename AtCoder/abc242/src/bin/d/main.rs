use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let s = scan!(String);
    let s: Vec<char> = s.chars().collect();
    let q = scan!(usize);
    for _ in 0..q {
        let (t, k) = scan!((usize, usize));
        let ans = f(t, k - 1, &s);
        println!("{}", ans);
    }
}

fn f(t: usize, k: usize, s: &[char]) -> char {
    if t == 0 {
        assert!(k < s.len());
        return s[k];
    }
    if k == 0 {
        return match t % 3 {
            0 => s[0],
            1 => suc(s[0]),
            2 => suc(suc(s[0])),
            _ => unreachable!(),
        };
    }
    let p = f(t - 1, k / 2, s);
    trans(p)[k % 2]
}

fn suc(ch: char) -> char {
    match ch {
        'A' => 'B',
        'B' => 'C',
        'C' => 'A',
        _ => unreachable!(),
    }
}

fn trans(ch: char) -> [char; 2] {
    match ch {
        'A' => ['B', 'C'],
        'B' => ['C', 'A'],
        'C' => ['A', 'B'],
        _ => unreachable!(),
    }
}
