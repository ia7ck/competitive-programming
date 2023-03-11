pub use __cargo_equip::prelude::*;

use proconio::{input, marker::Usize1};
use union_find::UnionFind;

fn main() {
    input! {
        n: usize,
        m: usize,
        abcd: [(Usize1, char, Usize1, char); m],
    };

    let vv = |v: usize, c: char| {
        if c == 'R' {
            v
        } else {
            n + v
        }
    };
    let mut uf = UnionFind::new(n * 2);
    for i in 0..n {
        uf.unite(vv(i, 'R'), vv(i, 'B'));
    }
    for &(a, b, c, d) in &abcd {
        uf.unite(vv(a, b), vv(c, d));
    }

    let mut e = vec![0; n * 2];
    for &(a, b, _, _) in &abcd {
        e[uf.find(vv(a, b))] += 1;
    }
    let mut ans1 = 0;
    let mut ans2 = 0;
    for i in 0..n * 2 {
        if uf.find(i) == i {
            if uf.get_size(i) == e[i] * 2 {
                ans1 += 1;
            } else {
                ans2 += 1;
            }
        }
    }
    println!("{} {}", ans1, ans2);
}

// The following code was expanded by `cargo-equip`.

///  # Bundled libraries
/// 
///  - `union_find 0.1.0 (git+https://github.com/ia7ck/rust-competitive-programming#5273e2d6570a53eaaf99a41e2dfcc797c918a6ea)` licensed under **missing** as `crate::__cargo_equip::crates::union_find`
#[cfg_attr(any(), rustfmt::skip)]
#[allow(unused)]
mod __cargo_equip {
    pub(crate) mod crates {
        pub mod union_find {pub struct UnionFind{par:Vec<usize>,size:Vec<usize>,}impl UnionFind{pub fn new(n:usize)->UnionFind{UnionFind{par:(0..n).collect(),size:vec![1;n],}}pub fn find(&mut self,i:usize)->usize{if self.par[i]!=i{self.par[i]=self.find(self.par[i]);}self.par[i]}pub fn unite(&mut self,i:usize,j:usize){let i=self.find(i);let j=self.find(j);if i==j{return;}let(i,j)=if self.size[i]>=self.size[j]{(i,j)}else{(j,i)};self.par[j]=i;self.size[i]+=self.size[j];}pub fn get_size(&mut self,i:usize)->usize{let p=self.find(i);self.size[p]}pub fn same(&mut self,i:usize,j:usize)->bool{self.find(i)==self.find(j)}}}
    }

    pub(crate) mod macros {
        pub mod union_find {}
    }

    pub(crate) mod prelude {pub use crate::__cargo_equip::crates::*;}

    mod preludes {
        pub mod union_find {}
    }
}
