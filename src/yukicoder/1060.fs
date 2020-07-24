// Learn more about F# at http://fsharp.org

open System

[<EntryPoint>]
let main argv =
    let [| n; m |] = stdin.ReadLine().Split() |> Array.map int

    let a =
        [| for _ in 1 .. n -> stdin.ReadLine().Split() |> Array.map int64 |]

    let s =
        [| for j in 0 .. m - 1 ->
            [ for i in 0 .. n - 1 -> a.[i].[j] ]
            |> List.sum |]

    let res =
        a
        |> Array.map (fun box ->
            (Array.zip box s) |> Array.sumBy (fun (cnt, tot) -> cnt * tot))
        |> Array.sortDescending
        |> Array.mapi (fun i pt ->
            if i % 2 = 0 then pt else -pt)
        |> Array.sum

    printfn "%d" res
    0 // return an integer exit code
