// Learn more about F# at http://fsharp.org

open System

[<EntryPoint>]
let main _ =
    let s = stdin.ReadLine().Replace("ao", "ki")
    printfn "%s" s
    0 // return an integer exit code
