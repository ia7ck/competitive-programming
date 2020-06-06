open System

let mutable mo = 1000000007L
let inline (+%) x y = (x + y) % mo
let inline ( *% ) x y = x * y % mo

[<EntryPoint>]
let main _ =
    let [| n; m; k |] = stdin.ReadLine().Split(' ') |> Array.map int64
    let line = stdin.ReadLine().Split(' ')
    let op = Array.head line |> Seq.head
    let b = Array.tail line |> Array.map int64

    let a =
        [| for _ in 1L .. n -> stdin.ReadLine() |> int64 |]

    mo <- k
    let sa = a |> Array.fold (+%) 0L
    let sb = b |> Array.fold (+%) 0L
    printfn "%d" <| if op = '+' then m *% sa +% n *% sb else sa *% sb
    0
