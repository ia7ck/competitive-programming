use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let mut a: Vec<u32> = rd.get_vec(n);
    let mut b: Vec<u32> = rd.get_vec(n);
    let mut c: Vec<u32> = rd.get_vec(n);

    a.sort();
    b.sort();
    c.sort();

    let mut j = 0;
    let mut k = 0;
    let mut ans = 0;
    for i in 0..n {
        while j < n && b[j] <= a[i] {
            j += 1;
        }
        if j == n {
            break;
        }
        assert!(b[j] > a[i]);
        while k < n && c[k] <= b[j] {
            k += 1;
        }
        if k == n {
            break;
        }
        assert!(c[k] > b[j]);
        ans += 1;
        j += 1;
        k += 1;
    }
    println!("{}", ans);
}

// 1 6  8  9 14
// 2 3 10 11 12
// 4 5  7 13 15
