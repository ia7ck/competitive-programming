import strutils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    k = read().parseInt
    s = read()
  if s.len <= k:
    echo s
  else:
    echo s[0..<k] & "..."

main()
