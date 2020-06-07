open System


[<EntryPoint>]
let main _ =
    let s = stdin.ReadLine()
    let ac = s.Split(',') |> Array.forall((=) "AC")
    printfn "%s" <| if ac then "Done!" else "Failed..."
    0
