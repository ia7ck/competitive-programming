use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let t: usize = rd.get();
    for _ in 0..t {
        let s: Vec<char> = rd.get_chars();
        solve(s);
    }
}

fn solve(s: Vec<char>) {
    let mut s: Vec<usize> = s.into_iter().map(|c| c as usize - 'a' as usize).collect();
    let mut cur = s.len();
    while !s.is_empty() {
        assert!(cur >= 1);
        if s[0] == cur - 1 {
            s.reverse();
            s.pop();
            s.reverse();
        } else if s[s.len() - 1] == cur - 1 {
            s.pop();
        } else {
            println!("NO");
            return;
        }
        cur -= 1;
    }
    println!("YES");
}
