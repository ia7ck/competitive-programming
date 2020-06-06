// Learn more about F# at http://fsharp.org

open System

let MOD = 1000000007L

let inline (+%) x y = (x + y) % MOD
let inline (-%) x y = (x - y + MOD) % MOD
let inline (%*) x y = (x * y) % MOD

[<EntryPoint>]
let main argv =
    let [| p; k |] = stdin.ReadLine().Split() |> Array.map int64

    let rec solve i dp0 dp1 =
        if i = k then
            dp0
        else
            let s = dp0 +% (dp1 %* (p -% 1L))
            solve (i + 1L) (s +% (dp0 %* p) +% (dp1 %* (p -% 1L))) (s +% dp1 %* (p -% 1L))

    let ans = solve 0L 1L 0L
    printfn "%d" ans
    0 // return an integer exit code
