open System

let sounds d =
    [ d
      d + 2
      d + 4
      d + 5
      d + 7
      d + 9
      d + 11 ]
    |> List.map (fun e -> e % 12)

[<EntryPoint>]
let main _ =
    let n = stdin.ReadLine() |> int
    let t = stdin.ReadLine().Split() |> Array.map int
    let ds =
        [ 0 .. 11 ]
        |> List.filter (fun d ->
            t |> Array.forall (fun u -> sounds d |> List.exists ((=) u)))
    if ds.IsEmpty || ds.Length >= 2 then
        printfn "-1"
    else
        printfn "%d" <| ds.Head
    0
