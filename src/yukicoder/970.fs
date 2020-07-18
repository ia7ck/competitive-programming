// Learn more about F# at http://fsharp.org

open System

[<EntryPoint>]
let main argv =
    let n = stdin.ReadLine() |> int
    let y = stdin.ReadLine().Split() |> Array.map int

    let s = y |> Array.sum
    y
    |> Array.map (fun v -> s - v - v * (n - 2))
    |> Array.iter (fun v -> printf "%d " v)
    0 // return an integer exit code
