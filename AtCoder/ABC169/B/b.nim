import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    n = read().parseInt
    a = newSeqWith(n, read().parseBiggestInt)

  if a.anyIt(it == 0):
    echo 0
    return
  var p = a[0]
  for x in a[1..^1]:
    if x > 1000000000000000000 div p:
      echo -1
      return
    p = p * x
  echo p
main()
