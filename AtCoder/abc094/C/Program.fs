let n = stdin.ReadLine() |> int
let a = stdin.ReadLine().Split() |> Array.map int

let left = a |> Array.sort |> Array.item (n / 2 - 1)
let right = a |> Array.sort |> Array.item (n / 2)

a
|> Array.map (fun x -> if x <= left then right else left)
|> Array.map string
|> String.concat "\n"
|> printfn "%s"
