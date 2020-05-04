// Learn more about F# at http://fsharp.org

open System

[<EntryPoint>]
let main argv =
    let s = stdin.ReadLine()
    let t = stdin.ReadLine()

    if t.Length = 1 && s |> String.exists (fun c -> c = t.[0]) then
        printfn "-1"
        exit 0
    let rec solve i =
        if i + t.Length > s.Length then 0
        elif s.[i..(i + t.Length - 1)] = t then 1 + solve (i + t.Length - 1)
        else solve (i + 1)
    solve 0 |> printfn "%A"
    0 // return an integer exit code
