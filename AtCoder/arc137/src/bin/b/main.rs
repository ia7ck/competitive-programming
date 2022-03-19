use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let n = scan!(usize);
    let a = scan!(i64; n);

    // [i, n)
    let mut right_one = vec![0; n + 1];
    for i in (0..n).rev() {
        if i + 1 < n {
            right_one[i] = right_one[i + 1];
        }
        if a[i] == 1 {
            right_one[i] += 1;
        }
    }

    let mut zero = 0_i64;
    let mut diff10 = 0;
    let mut diff01 = 0;
    let mut diff10_max = 0;
    let mut diff01_max = 0;
    let mut ans_max = 0;
    let mut ans_min = n;
    for i in 0..n {
        if a[i] == 0 {
            zero += 1;
            diff10 -= 1;
            diff01 += 1;
        } else {
            diff10 += 1;
            diff01 -= 1;
        }
        diff10_max = diff10_max.max(diff10);
        diff01_max = diff01_max.max(diff01);
        assert!(zero + diff10_max >= 0);
        ans_max = ans_max.max((zero + diff10_max) as usize + right_one[i + 1]);
        assert!(zero - diff01_max >= 0);
        ans_min = ans_min.min((zero - diff01_max) as usize + right_one[i + 1]);
    }
    let ans = ans_max - ans_min + 1;
    println!("{}", ans);
}
