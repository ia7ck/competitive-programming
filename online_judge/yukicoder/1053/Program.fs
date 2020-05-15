// Learn more about F# at http://fsharp.org

open System

let rec ok (a: List<int>) (seen: Set<int>) =
    match a |> List.tryHead with
    | None -> true
    | Some(x) ->
        if seen.Contains x
        then false
        else ok (a |> List.skipWhile (fun y -> y = x)) (seen.Add x)

[<EntryPoint>]
let main argv =
    let n = stdin.ReadLine() |> int

    let a =
        stdin.ReadLine().Split()
        |> Array.map (fun x -> int (x) - 1)
        |> Array.toList

    if (ok a (new Set<int>([]))) then
        printfn "0"
        exit 0

    let mutable freq = Array.create n 0
    for x in a do
        freq.[x] <- freq.[x] + 1
    match freq |> Array.tryFindIndex (fun v -> v >= 2) with
    | None -> printfn "0"
    | Some(x) ->
        let b =
            (a |> List.skipWhile (fun y -> y = x))
            @ (a |> List.takeWhile (fun y -> y = x))
        if (ok b (new Set<int>([]))) then printfn "1" else printfn "-1"
    0 // return an integer exit code
