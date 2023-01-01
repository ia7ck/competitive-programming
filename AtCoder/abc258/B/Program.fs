let n = stdin.ReadLine() |> int
let a = Array.init n (fun _ -> stdin.ReadLine())

let rec pow a x =
    assert (a >= 1UL)
    if x = 0UL then 1UL else a * (pow a (x - 1UL))

let rec f (i: int) (j: int) (di: int) (dj: int) (move: int) =
    let d = uint64 (a.[i].[j]) - uint64 ('0')

    if move = 0 then
        d
    else
        d * (pow 10UL (uint64 move)) + (f ((i + di + n) % n) ((j + dj + n) % n) di dj (move - 1))

let dirs = [ (0, 1); (-1, 1); (-1, 0); (-1, -1); (0, -1); (1, -1); (1, 0); (1, 1) ]

Seq.allPairs (seq { 0 .. (n - 1) }) (seq { 0 .. (n - 1) })
|> Seq.map (fun (i, j) -> dirs |> List.map (fun (di, dj) -> f i j di dj (n - 1)) |> List.max)
|> Seq.max
|> printfn "%d"
