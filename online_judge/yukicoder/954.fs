open System


[<EntryPoint>]
let main _ =
    let a =
        [|for _ in 1 .. 5 -> stdin.ReadLine() |> int64|]
        |> Array.rev

    let fib =
        (1L,1L)
        |> Seq.unfold(fun (f1,f2) ->
            if f1 > 1000000000000000L then None else Some(f1,(f2,f1 + f2)))
        |> Seq.toArray

    let ans =
        [for n in 1 .. 5 -> n]
        |> List.filter(fun n ->
            let b = a |> Array.take n
            [for i in 0 .. fib.Length - n -> Array.sub fib i n]
            |> List.exists((=) b))

    printfn "%d" <| if ans.Length = 0 then 0 else ans |> List.max
    0
