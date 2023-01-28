pub use __cargo_equip::prelude::*;

use graph::is_tree;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    };

    if !is_tree(n, &edges) {
        println!("No");
        return;
    }

    let mut deg = vec![0; n];
    for (u, v) in edges {
        deg[u] += 1;
        deg[v] += 1;
    }

    if deg.iter().all(|&d| d <= 2) {
        println!("Yes");
    } else {
        println!("No");
    }
}

// The following code was expanded by `cargo-equip`.

///  # Bundled libraries
/// 
///  - `graph 0.1.0 (git+https://github.com/ia7ck/rust-competitive-programming#52b241c96990e07c898ccdd85194983b72a776bd)` licensed under **missing** as `crate::__cargo_equip::crates::graph`
#[cfg_attr(any(), rustfmt::skip)]
#[allow(unused)]
mod __cargo_equip {
    pub(crate) mod crates {
        pub mod graph {pub fn is_tree(n:usize,edges:&[(usize,usize)])->bool{for&(a,b)in edges{assert!(a<n);assert!(b<n);}if n==0{return true;}edges.len()==n-1&&connectivity(n,edges)}pub fn connectivity(n:usize,edges:&[(usize,usize)])->bool{fn dfs(i:usize,g:&[Vec<usize>],visited:&mut Vec<bool>){for&j in&g[i]{if visited[j]{continue;}visited[j]=true;dfs(j,g,visited);}}let mut g=vec![vec![];n];for&(a,b)in edges{g[a].push(b);g[b].push(a);}let mut visited=vec![false;n];visited[0]=true;dfs(0,&g,&mut visited);visited.iter().filter(|&&f|f).count()==n}}
    }

    pub(crate) mod macros {
        pub mod graph {}
    }

    pub(crate) mod prelude {pub use crate::__cargo_equip::crates::*;}

    mod preludes {
        pub mod graph {}
    }
}
