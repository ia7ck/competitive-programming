import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let a, b, c, k = read().parseBiggestInt

  if k <= a:
    echo k
    return
  if k <= a + b:
    echo a
    return
  echo a - (k - a - b)

main()
