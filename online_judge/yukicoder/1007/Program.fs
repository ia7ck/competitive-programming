// Learn more about F# at http://fsharp.org

open System

[<EntryPoint>]
let main argv =
    let [| n; kk |] = stdin.ReadLine().Split() |> Array.map int
    let a = stdin.ReadLine().Split() |> Array.map int64

    let k = kk - 1
    if a.[k] = 0L then
        printfn "0"
        exit 0

    let pa =
        if k - 1 >= 0 then a.[..(k - 1)] else [||]

    let sa =
        if k + 1 < n then a.[(k + 1)..] else [||]

    let l =
        (max
            (match pa |> Array.tryFindIndexBack (fun x -> x = 0L) with
             | Some i -> i + 1
             | None -> 0)
             (match pa |> Array.tryFindIndexBack (fun x -> x = 1L) with
              | Some i -> i
              | None -> 0))

    let r =
        (min
            (match sa |> Array.tryFindIndex (fun x -> x = 0L) with
             | Some i -> i + k
             | None -> n - 1)
             (match sa |> Array.tryFindIndex (fun x -> x = 1L) with
              | Some i -> i + k + 1
              | None -> n - 1))

    if a.[k] = 1L then
        max (a.[l..k] |> Array.sum) (a.[k..r] |> Array.sum) |> printfn "%d"
    else
        a.[l..r]
        |> Array.sum
        |> printfn "%d"

    0 // return an integer exit code
