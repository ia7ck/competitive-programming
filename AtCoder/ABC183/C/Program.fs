// Learn more about F# at http://fsharp.org

open System

[<EntryPoint>]
let main _ =
    let [| n; k |] =
        stdin.ReadLine().Split(" ") |> Array.map int

    let t =
        Array.init n (fun _ -> stdin.ReadLine().Split(" ") |> Array.map int)

    let rec solve (seen: Set<_>) cur sum =
        if seen.Count = n then
            if sum + t.[cur].[0] = k then 1 else 0
        else
            [ 0 .. (n - 1) ]
            |> List.sumBy (fun nxt ->
                if seen.Contains nxt
                then 0
                else solve (seen.Add nxt) nxt (sum + t.[cur].[nxt]))

    solve (Set [ 0 ]) 0 0 |> printfn "%d"

    0 // return an integer exit code
