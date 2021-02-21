// Learn more about F# at http://fsharp.org

open System

[<EntryPoint>]
let main _ =
    let n = stdin.ReadLine() |> int

    for _ in 1 .. n do
        let d = stdin.ReadLine() |> int

        if d = 1 then
            "10"
        else
            let suf =
                String.concat "" (Seq.init (d - 1) (fun _ -> "0"))

            "9" + suf
        |> printfn "%s"

    0 // return an integer exit code
