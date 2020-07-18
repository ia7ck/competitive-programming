open System

let lowerbound (a: int array) (x: int) =
    // a.[ok] <= x
    let rec f ok ng =
        if ng - ok = 1 then
            ok
        else
            let mid = (ok + ng) / 2
            if a.[mid] <= x then f mid ng else f ok mid

    f -1 a.Length

[<EntryPoint>]
let main _ =
    let [| n; d |] = stdin.ReadLine().Split() |> Array.map int

    let a =
        [| for _ in 1 .. n -> stdin.ReadLine() |> int |]

    let b = a |> Array.sort

    a |> Array.iter (fun x -> printfn "%d" <| 1 + lowerbound b (x - d))
    0
