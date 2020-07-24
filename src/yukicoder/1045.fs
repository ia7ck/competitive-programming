// Learn more about F# at http://fsharp.org

open System

let max a b =
    if a > b then a else b

type S =
    { A: int
      B: int
      C: int }

let perm abc =
    let [| a; b; c |] = abc
    [| { A = a
         B = b
         C = c }
       { A = a
         B = c
         C = b }
       { A = b
         B = c
         C = a } |]

// x の上に y を置けるか
let fit (x: S) (y: S) = (x.A >= y.A && x.B >= y.B) || (x.A >= y.B && x.B >= y.A)

[<EntryPoint>]
let main argv =
    let n = stdin.ReadLine() |> int

    let rects =
        [| for i in 1 .. n -> stdin.ReadLine().Split() |> Array.map int |]
        |> Array.map perm

    let mutable dp = Array3D.create (1 <<< n) n 3 -1
    for i = 0 to (n - 1) do
        for j = 0 to 2 do
            dp.[1 <<< i, i, j] <- rects.[i].[j].C
    for (bits, i, j) in seq {
                            for bits in 0 .. ((1 <<< n) - 1) do
                                for i in 0 .. (n - 1) do
                                    for j in 0 .. 2 -> (bits, i, j)
                        } do
        if dp.[bits, i, j] > 0 then
            for k = 0 to (n - 1) do
                if ((bits >>> k) &&& 1) = 0 then
                    for l = 0 to 2 do
                        if fit rects.[i].[j] rects.[k].[l] then
                            dp.[bits ^^^ (1 <<< k), k, l] <- max dp.[bits ^^^ (1 <<< k), k, l]
                                                                 (dp.[bits, i, j] + rects.[k].[l].C)
    dp
    |> Seq.cast
    |> Seq.max
    |> printfn "%d"
    0 // return an integer exit code
