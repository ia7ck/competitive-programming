// Learn more about F# at http://fsharp.org

open System

[<EntryPoint>]
let main argv =
    let [| n; k |] = stdin.ReadLine().Split() |> Array.map int
    let s = stdin.ReadLine()

    let p = s.[..k - 2]
    s.[k - 1..] + (if (n - k + 1) % 2 = 0 then
                       p
                   else
                       p
                       |> Seq.rev
                       |> String.Concat)
    |> printf "%s"
    0 // return an integer exit code
