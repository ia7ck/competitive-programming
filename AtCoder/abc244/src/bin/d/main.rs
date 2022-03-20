use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let (s1, s2, s3) = scan!((char, char, char));
    let (t1, t2, t3) = scan!((char, char, char));

    let int = |ch| match ch {
        'R' => 0,
        'G' => 1,
        'B' => 2,
        _ => unreachable!(),
    };

    let inversion = |v: Vec<u8>| {
        let mut result = 0;
        for i in 0..v.len() {
            for j in (i + 1)..v.len() {
                if v[i] > v[j] {
                    result += 1;
                }
            }
        }
        result
    };

    let s_inv = inversion(vec![int(s1), int(s2), int(s3)]);
    let t_inv = inversion(vec![int(t1), int(t2), int(t3)]);

    if s_inv % 2 == t_inv % 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}

// R G B
// G R B

// RGB -> GRB -> GBR -> RBG
