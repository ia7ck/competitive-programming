// Learn more about F# at http://fsharp.org

open System

// v 円の紙幣 x 枚
let solve v x a =
    let b, x2 =
        a
        |> Array.sortDescending
        |> Array.mapFold (fun rest p ->
            (if rest <= 0 || p <= v then
                p, rest
             else
                 let k = Math.Min(rest, p / v)
                 p - k * v, rest - k)) x
    b
    |> Array.sortDescending
    |> Array.mapFold (fun rest p ->
        (if rest <= 0 then
            p, rest
         else
             let k =
                 Math.Min
                     (rest,
                      (if p = v then 2 else 1))
             p - k * v, rest - k)) x2
    |> fst
    |> Array.filter (fun p -> p >= 0)

[<EntryPoint>]
let main argv =
    let [| n; x; y; z |] = stdin.ReadLine().Split() |> Array.map int

    let a = stdin.ReadLine().Split() |> Array.map int

    let b = a |> solve 10000 z
    let c = b |> solve 5000 y
    let d = c |> solve 1000 x
    if d |> Array.isEmpty then printfn "Yes" else printfn "No"
    0 // return an integer exit code
