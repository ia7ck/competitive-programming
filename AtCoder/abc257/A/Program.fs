#nowarn "0025"

[<EntryPoint>]
let main _ =
    let [| n; x |] = stdin.ReadLine().Split() |> Array.map int

    let s =
        seq { 'A' .. 'Z' }
        |> Seq.map (fun ch -> String.replicate n (ch.ToString()))
        |> String.concat ""

    printfn "%c" s.[x - 1]

    0
