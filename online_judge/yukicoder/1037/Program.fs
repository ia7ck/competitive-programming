// Learn more about F# at http://fsharp.org

open System

type S =
    { X: int
      V: int
      W: int64 }

let min a b =
    if a < b then a else b

let max a b =
    if a > b then a else b

[<EntryPoint>]
let main argv =
    let [| n; v; l |] = stdin.ReadLine().Split() |> Array.map int

    let mutable stands =
        [| for i in 1 .. n ->
            (let [| x; v; w |] = stdin.ReadLine().Split() |> Array.map int
             { X = x
               V = v
               W = w |> int64 }) |]
    stands <-
        Array.append
            [| { X = 0
                 V = 0
                 W = 0 |> int64 } |] stands
    stands <-
        Array.append stands
            [| { X = l
                 V = 0
                 W = 0 |> int64 } |]
    let nn = stands.Length
    let inf = Int64.MaxValue / 2L
    let mutable dp = Array2D.create nn (v + 1) inf
    dp.[0, v] <- 0L
    for i = 1 to (nn - 1) do
        for j = 0 to v do
            let d = stands.[i].X - stands.[i - 1].X
            if j - d >= 0 then dp.[i, j - d] <- dp.[i - 1, j]
        for j = v downto 0 do
            let nj = min v (j + stands.[i].V)
            if nj > j then
                dp.[i, nj] <- min dp.[i, nj] (dp.[i, j] + stands.[i].W)
    // printfn "%A" dp
    // printfn "%A" stands
    let ans = dp.[nn - 1, *] |> Array.min
    if ans < inf then printfn "%d" ans else printfn "-1"
    0 // return an integer exit code
