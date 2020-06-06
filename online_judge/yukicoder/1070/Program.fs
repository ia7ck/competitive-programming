// Learn more about F# at http://fsharp.org

open System

[<EntryPoint>]
let main argv =
    let c = stdin.ReadLine()
    c
    |> Seq.tail
    |> Seq.filter (fun x -> x <> '0')
    |> Seq.length
    |> printfn "%d"
    0 // return an integer exit code
