[<EntryPoint>]
let main _ =
    let n = stdin.ReadLine() |> int
    let a = stdin.ReadLine().Split() |> Array.map int

    let b = Array.concat [ [| 0 |]; a; [| 0 |] ]
    let full = b |> Array.windowed 2 |> Array.sumBy (fun w -> abs (w.[1] - w.[0]))

    seq { 1..n }
    |> Seq.map (fun i ->
        full - abs (b.[i] - b.[i - 1]) - abs (b.[i + 1] - b.[i])
        + abs (b.[i + 1] - b.[i - 1]))
    |> Seq.map string
    |> String.concat "\n"
    |> printfn "%s"


    0
