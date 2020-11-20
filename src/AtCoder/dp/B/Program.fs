// Learn more about F# at http://fsharp.org

open System
open System.Collections.Generic

let memorec f =
    let memo = Dictionary<_, _>()

    let rec frec a =
        match memo.TryGetValue a with
        | exist, value when exist -> value
        | _ ->
            let value = f frec a
            memo.Add(a, value)
            value

    frec

[<EntryPoint>]
let main _ =

    let [| n; k |] =
        stdin.ReadLine().Split() |> Array.map int

    let h =
        stdin.ReadLine().Split() |> Array.map int

    let f =
        fun solve i ->
            if i = 0 then
                0
            else
                seq { i - k .. i - 1 }
                |> Seq.filter (fun j -> j >= 0)
                |> Seq.map (fun j -> abs (h.[j] - h.[i]) + (solve j))
                |> Seq.min

    printfn "%d" ((memorec f) (n - 1))
    0 // return an integer exit code
