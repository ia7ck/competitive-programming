use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let tlr: Vec<(u8, u32, u32)> = (0..n)
        .map(|_| {
            let t: u8 = rd.get();
            let l: u32 = rd.get();
            let r: u32 = rd.get();
            (t, l, r)
        })
        .collect();

    let f = |(t, l, r)| match t {
        1 => (l * 2, r * 2),
        2 => (l * 2, r * 2 - 1),
        3 => (l * 2 + 1, r * 2),
        4 => (l * 2 + 1, r * 2 - 1),
        _ => unreachable!(),
    };
    let mut ans = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            let (l1, r1) = f(tlr[i]);
            let (l2, r2) = f(tlr[j]);
            let ((_, r1), (l2, _)) = if l1 <= l2 {
                ((l1, r1), (l2, r2))
            } else {
                ((l2, r2), (l1, r1))
            };
            if l2 <= r1 {
                ans += 1
            }
        }
    }
    println!("{}", ans);
}
