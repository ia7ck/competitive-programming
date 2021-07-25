use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let a: Vec<u32> = rd.get_vec(n);
    let mut b: Vec<u32> = rd.get_vec(n);

    b.sort();
    let xs: Vec<u32> = b.iter().copied().map(|y| a[0] ^ y).collect();
    let mut ans = Vec::new();
    for x in xs {
        let mut c: Vec<u32> = a.iter().copied().map(|y| x ^ y).collect();
        c.sort();
        if b == c {
            ans.push(x);
        }
    }
    ans.sort();
    ans.dedup();
    println!("{}", ans.len());
    for ans in ans {
        println!("{}", ans);
    }
}
