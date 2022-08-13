use proconio::input;

fn main() {
    input! {
        r: usize,
        c: usize,
    };

    let rows = vec![
        "###############",
        "#.............#",
        "#.###########.#",
        "#.#.........#.#",
        "#.#.#######.#.#",
        "#.#.#.....#.#.#",
        "#.#.#.###.#.#.#",
        "#.#.#.#.#.#.#.#",
        "#.#.#.###.#.#.#",
        "#.#.#.....#.#.#",
        "#.#.#######.#.#",
        "#.#.........#.#",
        "#.###########.#",
        "#.............#",
        "###############",
    ];
    assert_eq!(rows.len(), 15);
    assert_eq!(rows[0].len(), 15);

    let a: Vec<Vec<char>> = rows.into_iter().map(|row| row.chars().collect()).collect();

    if a[r - 1][c - 1] == '#' {
        println!("black");
    } else {
        println!("white");
    }
}
