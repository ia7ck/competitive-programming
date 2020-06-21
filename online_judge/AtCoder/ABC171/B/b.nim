import strutils, sequtils, algorithm, math

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    n, k = read().parseInt
    a = newSeqWith(n, read().parseInt)

  echo a.sortedByIt(it)[0..<k].sum

main()
