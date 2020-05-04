// Learn more about F# at http://fsharp.org

open System

[<EntryPoint>]
let main argv =
    let x = stdin.ReadLine() |> int

    let d n =
        let mutable i = 1
        let mutable res = 0
        while i * i <= n do
            if n % i = 0 then
                if i * i = n then res <- res + 1 else res <- res + 2
            i <- i + 1
        res

    let f n = n - d n
    seq {
        for a = 1 to x - 1 do
            let b = x - a
            if (abs (a - b)) <= (x
                               |> float
                               |> sqrt
                               |> int) * 2 + 10
            then yield (abs (f a - f b), a, b)
    }
    |> Seq.groupBy (fun (y, _, _) -> y)
    |> Seq.minBy (fun (y, s) -> y)
    |> snd
    |> Seq.map (fun (y, a, b) -> (a, b))
    |> Seq.sort
    |> Seq.iter (fun (a, b) -> printfn "%d %d" a b)
    0 // return an integer exit code
