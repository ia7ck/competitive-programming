// Learn more about F# at http://fsharp.org

open System

let rec mpow (a: int64) x m =
    if x = 1L then a
    elif x % 2L = 0L then mpow (a * a % m) (x / 2L) m
    else a * mpow a (x - 1L) m % m

[<EntryPoint>]
let main argv =
    let [| n; m |] = stdin.ReadLine().Split() |> Array.map int
    let l = [ n; m ] |> List.max
    let mo = 1000000007L
    let fact =
        Array.concat
            [ [| 1L |]
              [| for i in 1L .. int64 (l) -> i |] ]
        |> Array.mapFold (fun acc x -> acc * x % mo, acc * x % mo) 1L
        |> fst
    let inv =
        Array.concat
            [ [| for i in 1L .. int64 (l) -> i |]
              [| 1L |] ]
        |> Array.rev
        |> Array.mapFold (fun acc x -> acc * x % mo, acc * x % mo) (mpow fact.[l] (mo - 2L) mo)
        |> fst
        |> Array.rev

    let binom a b = fact.[a] * inv.[b] % mo * inv.[a - b] % mo
    let mutable ans = 0L
    for i = m downto 1 do
        if (m - i) % 2 = 0
        then ans <- (ans + (binom m i) * (mpow (i |> int64) (n |> int64) mo) % mo) % mo
        else ans <- (ans - (binom m i) * (mpow (i |> int64) (n |> int64) mo) % mo + mo) % mo
    printfn "%d" ans
    0 // return an integer exit code
