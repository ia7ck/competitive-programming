#nowarn "0025"

let [| x; y; z |] = stdin.ReadLine().Split() |> Array.map int

let ans =
    if x < y && y < 0 then
        if y < z then
            Some(abs (z) + abs (y - z) + abs (x - y))
        else
            None
    elif 0 < y && y < x then
        if z < y then
            Some(abs (z) + abs (y - z) + abs (x - y))
        else
            None
    else
        Some(abs(x))

ans |> Option.defaultValue -1 |> printfn "%d"
