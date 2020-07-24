// Learn more about F# at http://fsharp.org

open System

let mo = 1000000007L
let inline (+%=) (x: byref<int64>) y = x <- (x + y) % mo
let inline ( *% ) x y = x * y % mo

type mat = int64 [] []

let inline ( *@ ) (a: mat) (b: mat) =
    let n = a.Length
    let mutable c = Array.init n (fun _ -> Array.create n 0L)
    for i = 0 to n - 1 do
        for j = 0 to n - 1 do
            for k = 0 to n - 1 do
                &c.[i].[j] +%= a.[i].[k] *% b.[k].[j]
    c

let rec matpow a n =
    if n = 1 then a
    elif n % 2 = 0 then matpow (a *@ a) (n / 2)
    else a *@ matpow a (n - 1)

[<EntryPoint>]
let main argv =
    let [| m; k |] = stdin.ReadLine().Split() |> Array.map int
    let mutable a = Array.init m (fun _ -> Array.create m 1L)
    for n = 0 to m - 1 do
        for i = 0 to m - 1 do
            &a.[n].[n * i % m] +%= 1L

    let b = (matpow a k)
    printfn "%d" b.[0].[0]
    0 // return an integer exit code
