use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut b = vec![false; 4];
    let mut p = 0;
    for a in a {
        assert_eq!(b[0], false);
        b[0] = true;
        let mut c = vec![false; 4];
        for j in 0..=3 {
            if b[j] {
                if a + j >= 4 {
                    p += 1;
                } else {
                    c[a + j] = b[j];
                }
            }
        }
        b = c;
    }

    println!("{}", p);
}
