use proconio::input;

fn set_eq(xy: &[(i64, i64)], uv: &[(i64, i64)]) -> bool {
    let mut xy = xy.to_vec();
    let mut uv = uv.to_vec();
    xy.sort();
    uv.sort();
    xy == uv
}

fn solve(xy: &[(i64, i64)], uv: &[(i64, i64)]) -> bool {
    let (x, _) = xy[0];
    let mut ys = xy
        .iter()
        .copied()
        .filter(|&(xx, _)| xx == x)
        .map(|(_, y)| y)
        .collect::<Vec<_>>();
    let mut vs = uv
        .iter()
        .copied()
        .filter(|&(u, _)| u == x)
        .map(|(_, v)| v)
        .collect::<Vec<_>>();

    if vs.is_empty() {
        return false;
    }

    ys.sort();
    vs.sort();
    let m = (ys[0] + vs[vs.len() - 1]) / 2;
    let xy = xy
        .iter()
        .copied()
        .map(|(x, y)| (x, m * 2 - y))
        .collect::<Vec<_>>();
    set_eq(&xy, &uv)
}

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
        uv: [(i64, i64); n],
    }

    let xy = xy
        .into_iter()
        .map(|(x, y)| (x * 2, y * 2))
        .collect::<Vec<_>>();
    let uv = uv
        .into_iter()
        .map(|(u, v)| (u * 2, v * 2))
        .collect::<Vec<_>>();

    let yx = xy.iter().copied().map(|(x, y)| (y, x)).collect::<Vec<_>>();
    let vu = uv.iter().copied().map(|(u, v)| (v, u)).collect::<Vec<_>>();

    if set_eq(&xy, &uv) || solve(&xy, &uv) || solve(&yx, &vu) {
        println!("Yes");
    } else {
        println!("No");
    }
}
