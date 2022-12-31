[<EntryPoint>]
let main _ =
    let n = stdin.ReadLine() |> int
    let a = stdin.ReadLine().Split() |> Array.map int

    let x =
        Array.windowed 2 a
        |> Array.tryFind (fun w -> w.[0] > w.[1])
        |> Option.map (fun w -> w.[0])
        |> Option.defaultValue a.[n - 1]

    let b = a |> Array.filter (fun y -> x <> y)

    b |> Array.map string |> String.concat " " |> printfn "%s"
    0
