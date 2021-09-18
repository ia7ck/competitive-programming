use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let x: String = rd.get();
    let mut map = vec![0; 26];
    let mut inv = vec![0; 26];
    for (i, c) in x.chars().enumerate() {
        let j = c as usize - 'a' as usize;
        map[j] = i;
        inv[i] = j;
    }
    let n: usize = rd.get();
    let ss: Vec<String> = (0..n).map(|_| rd.get()).collect();
    let mut ss: Vec<String> = ss
        .into_iter()
        .map(|s| {
            s.chars()
                .map(|c| {
                    let j = c as usize - 'a' as usize;
                    (map[j] as u8 + 'a' as u8) as char
                })
                .collect()
        })
        .collect();
    ss.sort();
    let ss: Vec<String> = ss
        .into_iter()
        .map(|s| {
            s.chars()
                .map(|c| {
                    let j = c as usize - 'a' as usize;
                    (inv[j] as u8 + 'a' as u8) as char
                })
                .collect()
        })
        .collect();
    for s in ss {
        println!("{}", s);
    }
}
