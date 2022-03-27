use scanner_proc_macro::insert_scanner;
use std::collections::{BTreeSet, HashMap};

#[insert_scanner]
fn main() {
    let (n, m) = scan!((usize, usize));
    let a = scan!(u32; n);
    let b = scan!(u32; n);
    let c = scan!(u32; m);
    let d = scan!(u32; m);

    let mut chocolate_y = HashMap::new();
    for i in 0..n {
        chocolate_y.entry(a[i]).or_insert(Vec::new()).push(b[i]);
    }
    let mut slot_yj = HashMap::new();
    for j in 0..m {
        slot_yj.entry(c[j]).or_insert(Vec::new()).push((d[j], j));
    }

    let mut xs = Vec::new();
    for i in 0..n {
        xs.push(a[i]);
    }
    for j in 0..m {
        xs.push(c[j]);
    }
    xs.sort();
    xs.dedup();
    xs.reverse();

    let mut ok_slot_yj = BTreeSet::new();
    for x in xs {
        if let Some(yj) = slot_yj.get(&x) {
            for yj in yj {
                ok_slot_yj.insert(yj);
            }
        }
        if let Some(y) = chocolate_y.get(&x) {
            for &y in y {
                if let Some(yj) = ok_slot_yj.range((y, 0)..).next().copied() {
                    ok_slot_yj.remove(yj);
                } else {
                    println!("No");
                    return;
                }
            }
        }
    }
    println!("Yes");
}
