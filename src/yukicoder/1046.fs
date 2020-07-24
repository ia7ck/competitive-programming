open System

[<EntryPoint>]
let main _ =
    let [| n; k |] = stdin.ReadLine().Split(' ') |> Array.map int
    let a = stdin.ReadLine().Split(' ') |> Array.map int

    if a |> Array.forall (fun x -> x < 0) then
        a
        |> Array.max
        |> printfn "%d"
    else
        a
        |> Array.sortDescending
        |> Array.take k
        |> Array.takeWhile (fun x -> x >= 0)
        |> Array.sum
        |> printfn "%d"
    0
