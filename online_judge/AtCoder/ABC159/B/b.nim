import strutils, sequtils, algorithm

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc palin(s: string): bool =
  for i in 0..<(s.len div 2):
    if s[i] != s[s.len - i - 1]:
      return false
  return true

proc main() =
  let
    s = read()
    n = s.len

  let ok = palin(s) and
    palin(s[0..<((n - 1) div 2)]) and
    palin(s[((n + 3) div 2 - 1)..<n])
  if ok:
    echo "Yes"
  else:
    echo "No"

main()
