import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    s = read()
    t = read()

  var ans = 0
  for i in 0..<s.len:
    if s[i] != t[i]:
      ans += 1
  echo ans
main()
