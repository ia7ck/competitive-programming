// Learn more about F# at http://fsharp.org

open System

[<EntryPoint>]
let main argv =
    let n = stdin.ReadLine() |> int

    let h =
        stdin.ReadLine().Split() |> Array.map int

    let inf = 1_000_000_000
    let memo = Array.init n (fun _ -> inf)

    let rec solve i =
        if i <= 0 then
            0
        elif i = 1 then
            abs (h.[0] - h.[1])
        elif memo.[i] < inf then
            memo.[i]
        else
            memo.[i] <- min (abs (h.[i - 1] - h.[i]) + solve (i - 1)) (abs (h.[i - 2] - h.[i]) + solve (i - 2))
            memo.[i]

    solve (n - 1) |> printfn "%d"
    0 // return an integer exit code
