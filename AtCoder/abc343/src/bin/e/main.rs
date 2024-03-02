use proconio::input;

fn main() {
    input! {
        v1: u32,
        v2: u32,
        v3: u32,
    };

    let cube1 = Cuboid::new((0, 0, 0), (7, 7, 7));
    for a2 in -14..=7 {
        for b2 in -14..=7 {
            for c2 in -14..=7 {
                let cube2 = Cuboid::new((a2, b2, c2), (a2 + 7, b2 + 7, c2 + 7));
                let cube12 = cube1.intersect(&cube2);
                for a3 in -14..=7 {
                    for b3 in -14..=7 {
                        for c3 in -14..=7 {
                            let cube3 = Cuboid::new((a3, b3, c3), (a3 + 7, b3 + 7, c3 + 7));
                            let cube13 = cube1.intersect(&cube3);
                            let cube23 = cube2.intersect(&cube3);
                            let cube123 = cube12.and_then(|cube| cube.intersect(&cube3));

                            let u3 = cube123.map(|cube| cube.volume()).unwrap_or(0);
                            let u2 = cube12.map(|cube| cube.volume()).unwrap_or(0)
                                + cube13.map(|cube| cube.volume()).unwrap_or(0)
                                + cube23.map(|cube| cube.volume()).unwrap_or(0)
                                - u3 * 3;
                            let u1 =
                                cube1.volume() + cube2.volume() + cube3.volume() - u2 * 2 - u3 * 3; // ???
                            if (u1, u2, u3) == (v1, v2, v3) {
                                println!("Yes");
                                println!("0 0 0 {} {} {} {} {} {}", a2, b2, c2, a3, b3, c3);
                                return;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("No");
}

#[derive(Debug, Clone, Copy)]
struct Cuboid {
    sx: i32,
    sy: i32,
    sz: i32,
    tx: i32,
    ty: i32,
    tz: i32,
}

impl Cuboid {
    fn new((sx, sy, sz): (i32, i32, i32), (tx, ty, tz): (i32, i32, i32)) -> Self {
        assert!(sx < tx);
        assert!(sy < ty);
        assert!(sz < tz);
        Self {
            sx,
            sy,
            sz,
            tx,
            ty,
            tz,
        }
    }

    fn volume(&self) -> u32 {
        ((self.tx - self.sx) * (self.ty - self.sy) * (self.tz - self.sz)) as u32
    }

    fn intersect(&self, other: &Self) -> Option<Self> {
        let sx = self.sx.max(other.sx);
        let sy = self.sy.max(other.sy);
        let sz = self.sz.max(other.sz);
        let tx = self.tx.min(other.tx);
        let ty = self.ty.min(other.ty);
        let tz = self.tz.min(other.tz);
        if sx < tx && sy < ty && sz < tz {
            Some(Self::new((sx, sy, sz), (tx, ty, tz)))
        } else {
            None
        }
    }
}
