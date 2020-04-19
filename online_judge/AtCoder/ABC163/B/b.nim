import strutils, sequtils, math

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let
    n, m = read().parseInt
    a = newSeqWith(m, read().parseInt)
    s = a.sum
  if n < s:
    echo -1
  else:
    echo n - s

main()
