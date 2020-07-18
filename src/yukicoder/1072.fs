// Learn more about F# at http://fsharp.org

open System

let get (map: Map<int, int64>) (key: int) =
    match map.TryFind key with
    | Some f -> f
    | None -> 0L

[<EntryPoint>]
let main argv =
    let [| n; x |] = stdin.ReadLine().Split() |> Array.map int

    let a =
        [ for i in 1 .. n -> stdin.ReadLine() |> int ]

    let res =
        a
        |> List.fold (fun (freq: Map<int, int64>, ans) y ->
            let z = x ^^^ y
            let zf = get freq z
            let yf = get freq y
            freq.Add(y, yf + 1L), ans + zf) (Map.empty, 0L)

    res
    |> snd
    |> printfn "%d"
    0 // return an integer exit code
