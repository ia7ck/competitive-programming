use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let a: i64 = rd.get();
    let b: i64 = rd.get();
    let c: i64 = rd.get();

    let d = b - a;
    let e = c - b;
    let f = (d - e).abs();
    if (d >= 0 && e >= 0) || (d <= 0 && e <= 0) {
        if d <= e {
            if f % 2 == 0 {
                println!("{}", f / 2);
            } else {
                println!("{}", f / 2 + 2);
            }
        } else {
            println!("{}", f);
        }
    } else if d < 0 && e > 0 {
        if f % 2 == 0 {
            println!("{}", f / 2);
        } else {
            println!("{}", f / 2 + 2);
        }
    } else {
        println!("{}", d.abs() + e.abs());
    }
}
