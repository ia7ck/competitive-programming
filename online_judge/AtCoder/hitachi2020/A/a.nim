import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    s = read()
    n = s.len
  if n mod 2 == 1:
    echo "No"
    return
  let t = "hi".repeat(n div 2)
  if s == t:
    echo "Yes"
  else:
    echo "No"

main()
