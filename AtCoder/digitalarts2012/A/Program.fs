let words = stdin.ReadLine().Split()
let n = stdin.ReadLine() |> int
let ng_words = List.init n (fun _ -> stdin.ReadLine())

let test (text: string) (pattern: string) =
    text.Length = pattern.Length
    && (text, pattern) ||> Seq.forall2 (fun ch1 ch2 -> ch1 = ch2 || ch2 = '*')

words
|> Array.map (fun w ->
    if List.exists (test w) ng_words then
        String.replicate w.Length "*"
    else
        w)
|> String.concat " "
|> printfn "%s"
