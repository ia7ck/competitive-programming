// Learn more about F# at http://fsharp.org

open System
open System.Collections.Generic

let memorec f =
    let memo = Dictionary<_, _>()

    let rec frec a =
        match memo.TryGetValue a with
        | exist, value when exist -> value
        | _ ->
            let value = f frec a
            memo.Add(a, value)
            value

    frec

open System.Numerics

let f =
    fun solve n k ->
        if k = 0 then
            // 残り上の桁全て 0 で埋める
            1I
        elif n < 10I then
            if k = 1 then n else 0I
        else
            let q = n / 10I
            let r = n % 10I

            (solve q k)
            + r * (solve q (k - 1))
            + (9I - r) * (solve (q - 1I) (k - 1))

[<EntryPoint>]
let main _ =
    let n = stdin.ReadLine() |> BigInteger.Parse
    let k = stdin.ReadLine() |> int

    printfn "%A" ((memorec f) n k)

    0 // return an integer exit code
