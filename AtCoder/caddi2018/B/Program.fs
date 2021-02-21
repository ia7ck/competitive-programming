// Learn more about F# at http://fsharp.org

open System

[<EntryPoint>]
let main argv =
    let n = stdin.ReadLine() |> int
    let a = [ 1 .. n ] |> List.map (fun _ -> stdin.ReadLine() |> int)

    let ans =
        if a |> List.forall (fun x -> x % 2 = 0) then "second" else "first"

    printfn "%s" ans
    0 // return an integer exit code
