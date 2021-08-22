use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: i32 = rd.get();

    let ans = if n == 2015 {
        1
    } else if n == 2016 {
        2
    } else if n == 2017 {
        -1
    } else if n >= 2018 {
        n - 2018 + 3
    } else {
        -1
    };
    println!("{}", ans);
}
