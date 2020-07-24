// Learn more about F# at http://fsharp.org

open System

[<EntryPoint>]
let main argv =
    let [| xx; yy; h |] = stdin.ReadLine().Split() |> Array.map float
    let x, y = Math.Min(xx, yy), Math.Max(xx, yy)

    let ans =
        seq {
            for a in 0 .. 20 do
                // x を a 回折れるか
                // x / 2^(a - 1) > h * 2^(a - 1)
                let p2 = Math.Pow(2.0, float (a) - 1.0)
                if a = 0 || x * 1000.0 / p2 > h * p2 then
                    for b in 0 .. 20 do
                        let q2 = Math.Pow(2.0, float (b) - 1.0)
                        if b = 0
                           || y * 1000.0 / q2 > h * Math.Pow(2.0, float (a))
                                                * q2 then yield (a, b)
        }
        |> Seq.map (fun tup -> (fst tup) + (snd tup))
        |> Seq.max
    printfn "%d" ans
    0 // return an integer exit code
