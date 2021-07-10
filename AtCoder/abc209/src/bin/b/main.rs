use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let x: u32 = rd.get();
    let a: Vec<u32> = rd.get_vec(n);

    let mut y = 0;
    for i in 0..n {
        if i % 2 == 0 {
            y += a[i];
        } else {
            y += a[i] - 1;
        }
    }
    if y <= x {
        println!("Yes");
    } else {
        println!("No");
    }
}
