// Learn more about F# at http://fsharp.org

open System

[<EntryPoint>]
let main argv =
    // https://docs.microsoft.com/ja-jp/dotnet/fsharp/language-reference/pattern-matching#array-pattern
    let [|a; b; c; d|] = 
        stdin.ReadLine().Split() 
        |> Array.map int 
        |> Array.sortBy(id)
    if a + 1 = b && b + 1 = c && c + 1 = d then
        printfn "Yes"
    else
        printfn "No"
    0 // return an integer exit code
