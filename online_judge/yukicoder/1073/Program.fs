// Learn more about F# at http://fsharp.org

open System

let mo = 1000000007L
let inline (+%) x y = (x + y) % mo
let inline (+%=) (x: byref<int64>) y = x <- x +% y
let inline ( *% ) x y = x * y % mo

let rec modpow a n =
    if n = 0 then 1L
    elif n = 1 then a
    elif n % 2 = 0 then modpow (a *% a) (n / 2)
    else a *% modpow a (n - 1)

let inline (/%) x y = x *% (modpow y ((int) mo - 2))

type Mat = int64 [] []

let inline ( *@ ) (a: Mat) (b: Mat) =
    let n = a.Length
    let mutable c: Mat = Array.init n (fun _ -> Array.create n 0L)
    for i = 0 to n - 1 do
        for j = 0 to n - 1 do
            for k = 0 to n - 1 do
                &c.[i].[j] +%= a.[i].[k] *% b.[k].[j]
    c

let rec matpow a n =
    if n = 1L then a
    elif n % 2L = 0L then matpow (a *@ a) (n / 2L)
    else a *@ matpow a (n - 1L)

[<EntryPoint>]
let main argv =
    let n = stdin.ReadLine() |> int64

    let inv6 = 1L /% 6L

    let p =
        Array.create 6 0L
        |> Array.mapFold (fun acc _ ->
            let y = acc *% inv6 +% inv6
            y, acc +% y) 0L
        |> fst
        |> Array.append [| 1L |]
        |> Array.take 6

    if n < 6L then
        printfn "%d" p.[(int) n]
    else
        let a: Mat =
            [| [| 0L; 1L; 0L; 0L; 0L; 0L |]
               [| 0L; 0L; 1L; 0L; 0L; 0L |]
               [| 0L; 0L; 0L; 1L; 0L; 0L |]
               [| 0L; 0L; 0L; 0L; 1L; 0L |]
               [| 0L; 0L; 0L; 0L; 0L; 1L |]
               [| inv6; inv6; inv6; inv6; inv6; inv6 |] |]

        let b = matpow a (n - 5L)

        let q =
            [| for i in 0 .. 5 -> Array.map2 ( *% ) b.[i] p |> Array.fold (+%) 0L |]
        printfn "%d" (Array.last q)
    0 // return an integer exit code
