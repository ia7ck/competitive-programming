import strutils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    s = read()
    t = read()
  if s == t[0..<s.len]:
    echo "Yes"
  else:
    echo "No"

main()
