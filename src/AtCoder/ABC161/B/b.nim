import strutils, sequtils, math

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    n, m = read().parseInt
    a = newSeqWith(n, read().parseInt)
  let s = a.sum
  var c = 0
  for it in a:
    if it * 4 * m >= s:
      c += 1
  if c >= m:
    echo "Yes"
  else:
    echo "No"
main()
