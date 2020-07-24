open System

[<EntryPoint>]
let main _ =
    let [| n; k; x; y |] = stdin.ReadLine().Split(' ') |> Array.map int64
    let a = stdin.ReadLine().Split(' ') |> Array.map int64

    let b =
        a
        |> Array.map (fun e -> (e - 1L + k - 1L) / k)
        |> Array.sort

    let (_, imi, maraa) =
        b
        |> Array.fold
            (fun (i, imi, maraa) e -> if y <= x * i then (i - 1L, imi, e) else (i - 1L, imi + e - maraa, maraa))
               (n, 0L, 0L)

    imi * x + maraa * y |> printfn "%d"
    0
