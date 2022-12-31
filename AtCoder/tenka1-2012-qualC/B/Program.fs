open System.Collections.Generic

let marks = [ "S"; "H"; "D"; "C" ]
let numbers = [ "A"; "2"; "3"; "4"; "5"; "6"; "7"; "8"; "9"; "10"; "J"; "Q"; "K" ]


let rec parse (l: char list) =
    match l with
    | mark :: '1' :: '0' :: tail ->
        assert (List.contains (string (mark)) marks)
        (sprintf "%c10" mark) :: parse (tail)
    | mark :: number :: tail ->
        assert (List.contains (string (mark)) marks)
        assert (List.contains (string (number)) numbers)
        (sprintf "%c%c" mark number) :: parse (tail)
    | [] -> []
    | _ -> failwithf "parse error: %A" l

let cards = stdin.ReadLine() |> Seq.toList |> parse

let ans =
    marks
    |> List.map (fun mark ->
        let sub = List.map ((+) mark) [ "10"; "J"; "Q"; "K"; "A" ] in
        let seen = new HashSet<string>() in

        cards
        |> List.takeWhile (fun card ->
            seen.Add(card) |> ignore
            not (List.forall seen.Contains sub))
        |> List.except sub)
    |> List.minBy List.length
    |> List.map string
    |> String.concat ""

printfn "%s" (if ans.Length = 0 then "0" else ans)
