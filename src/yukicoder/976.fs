// Learn more about F# at http://fsharp.org

open System

[<EntryPoint>]
let main argv =
    let m = stdin.ReadLine() |> int64
    let ans = [1 .. 128] |> List.fold (fun acc _ -> acc * 2L % m) 1L
    printfn "%d" ans
    0 // return an integer exit code
