open System

let factors n =
    seq {
        for i in 1L .. n -> i
    }
    |> Seq.takeWhile (fun i -> i * i <= n)
    |> Seq.filter (fun i -> n % i = 0L)
    |> Seq.collect (fun i ->
        seq {
            yield i
            yield n / i
        })
    |> Seq.sort
    |> Seq.distinct

let sqrt n =
    let rec solve sm bg = // sm * sm <= n < bg * bg
        if bg - sm <= 1L then
            sm
        else
            let mid = (bg + sm) / 2L
            if mid * mid <= n then solve mid bg else solve sm mid
    solve 0L (n + 1L)

[<EntryPoint>]
let main _ =
    let n = stdin.ReadLine() |> int64

    let bs =
        factors n
        |> Seq.filter (fun b ->
            let m = (n / b)
            (sqrt m) * (sqrt m) = m)

    let b =
        bs
        |> Seq.sort
        |> Seq.head

    let a = sqrt (n / b)

    printfn "%d %d" a b
    0
