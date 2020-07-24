open System

[<EntryPoint>]
let main _ =
    let [| a; b |] = stdin.ReadLine().Split(' ') |> Array.map int64

    let s =
        (0L, 0)
        |> Seq.unfold (fun (n, i) ->
            if abs i >= 3 then None else Some(n, (a * n + b, i + 1)))

    let i =
        s
        |> Seq.tail
        |> Seq.tryFindIndex ((=) 0L)

    match i with
    | Some i -> printfn "%d" <| i + 1
    | None -> printfn "-1"
    0
