use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let m: usize = rd.get();
    let k: usize = rd.get();

    let mut tail = Vec::new();
    let mut inv = 0;
    let mut a = Vec::new();
    for x in (0..n).rev() {
        if inv + x <= k {
            inv += x;
            a.push(x);
        } else {
            tail.push(x);
        }
    }
    tail.reverse();
    a.extend(tail);
    let s = a.iter().copied().sum::<usize>();
    assert!(s <= m);
    let i = a.iter().position(|&x| x == n - 1).unwrap();
    a[i] += m - s;
    for x in a {
        println!("{}", x);
    }
}
