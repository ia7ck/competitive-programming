// Learn more about F# at http://fsharp.org

open System

let solve n i j =
    let k = [i; n - i - 1L; j; n - j - 1L] |> List.min
    k * 4L * (n - k) + 
        if i = k then
            j - k
        elif j = n - k - 1L then
            n - k * 2L - 1L + i - k
        elif i = n - k - 1L then
            (n - k * 2L - 1L) * 2L + n - k - 1L - j
        else
            (n - k * 2L - 1L) * 3L + n - k - 1L - i

[<EntryPoint>]
let main argv =
    let q = stdin.ReadLine() |> int
    for qq = 1 to q do
        let [|n; i; j|] = stdin.ReadLine().Split() |> Array.map int64
        printfn "%d" (solve n i j)
    0 // return an integer exit code
