open System

// x <= arr[i]
let lowerBound x (arr: 'a []) =
    // arr[lf] < x <= arr[rg]
    let rec solve lf rg =
        if rg - lf <= 1 then
            rg
        else
            let mid = (rg + lf) / 2
            if arr.[mid] < x then solve mid rg else solve lf mid
    solve -1 arr.Length


[<EntryPoint>]
let main _ =
    let [| n; m; k |] = stdin.ReadLine().Split(' ') |> Array.map int64
    let line = stdin.ReadLine().Split(' ')
    let op = Array.head line |> Seq.head

    let b =
        Array.tail line
        |> Array.map int64
        |> Array.sort

    let a =
        [| for _ in 1L .. n -> stdin.ReadLine() |> int64 |]

    a
    |> Array.sumBy (fun x ->
        let y =
            if op = '+' then k - x else (k + x - 1L) /x

        let i =
            b
            |> lowerBound y
            |> int64

        m - i)
    |> printfn "%d"

    0
