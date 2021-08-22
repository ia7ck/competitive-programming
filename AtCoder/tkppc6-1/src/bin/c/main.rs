use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let a: Vec<i32> = rd.get_vec(n * 2 - 1);

    let lt = a.iter().filter(|&&x| x != -1 && x < a[0]).count();
    let gt = a.iter().filter(|&&x| x != -1 && x > a[0]).count();
    let eq = a.iter().filter(|&&x| x != -1 && x == a[0]).count();
    let free = a.iter().filter(|&&x| x == -1).count();
    let (lt, gt) = if lt >= gt { (lt, gt) } else { (gt, lt) };
    if lt - gt > eq + free {
        println!("No");
    } else {
        println!("Yes");
    }
}
