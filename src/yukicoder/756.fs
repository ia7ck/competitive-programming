// Learn more about F# at http://fsharp.org

open System

[<EntryPoint>]
let main argv =
    let d = stdin.ReadLine().Trim() |> int
    let s = 
        [1..100] 
        |> List.map (fun x -> x |> string) 
        |> List.fold (fun acc x -> acc + x) ""
    printfn "%c" s.[d - 1]
    0
