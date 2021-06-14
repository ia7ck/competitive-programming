use f::FordFulkerson;
use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let h: usize = rd.get();
    let w: usize = rd.get();
    let n: usize = rd.get();

    let mut g = FordFulkerson::new(h + n + n + w + 2);
    let source = h + n + n + w;
    let sink = source + 1;
    for i in 0..h {
        g.add_edge(source, i, 1);
    }
    for i in 0..n {
        g.add_edge(h + i, h + n + i, 1);
    }
    for i in 0..w {
        g.add_edge(h + n + n + i, sink, 1);
    }
    for i in 0..n {
        let a: usize = rd.get();
        let b: usize = rd.get();
        let c: usize = rd.get();
        let d: usize = rd.get();
        for j in (a - 1)..c {
            g.add_edge(j, h + i, 1);
        }
        for j in (b - 1)..d {
            g.add_edge(h + n + i, h + n + n + j, 1);
        }
    }
    let ans = g.max_flow(source, sink);
    println!("{}", ans);
}
