// Learn more about F# at http://fsharp.org

open System

let isPrime n =
    if n = 1L then
        false
    else
        seq { 2L .. n }
        |> Seq.filter (fun i -> i * i <= n)
        |> Seq.forall (fun i -> n % i <> 0L)

[<EntryPoint>]
let main argv =
    let n = stdin.ReadLine() |> int

    let primes =
        Seq.initInfinite (fun i -> i |> int64)
        |> Seq.skip 100000
        |> Seq.filter isPrime
        |> Seq.take 10

    let niceNumbers =
        Seq.singleton 1L
        |> Seq.append
            (primes
             |> Seq.collect (fun p -> (primes |> Seq.map (fun q -> p * q))))
        |> Seq.sort
        |> Seq.distinct

    niceNumbers
    |> Seq.item (n - 1)
    |> printfn "%d"
    0 // return an integer exit code
