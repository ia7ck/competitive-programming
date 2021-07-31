use binary_search::BinarySearch;
use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let m: usize = rd.get();
    let a: Vec<i32> = rd.get_vec(n);
    let mut b: Vec<i32> = rd.get_vec(m);

    let mut ans = (a[0] - b[0]).abs();
    b.sort();
    for x in a {
        let j = b.lower_bound(&x);
        if let Some(&y) = b.get(j) {
            ans = ans.min((x - y).abs());
        }
        if j >= 1 {
            if let Some(&y) = b.get(j - 1) {
                ans = ans.min((x - y).abs());
            }
        }
    }
    println!("{}", ans);
}
