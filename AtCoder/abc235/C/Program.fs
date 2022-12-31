#nowarn "0025"

let [| n; q |] = stdin.ReadLine().Split() |> Array.map int
let a = stdin.ReadLine().Split() |> Array.map int

let table =
    seq { 1..n } |> Array.ofSeq |> Array.groupBy (fun i -> a.[i - 1]) |> Map.ofArray

let solve (x: int) (k: int) =
    table
    |> Map.tryFind x
    |> Option.bind (Array.tryItem (k - 1))
    |> Option.defaultValue -1

seq { 1..q }
|> Seq.map (fun _ -> let [| x; k |] = stdin.ReadLine().Split() |> Array.map int in solve x k)
|> Seq.map string
|> String.concat "\n"
|> printfn "%s"
