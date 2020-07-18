import strutils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    a, v = read().parseBiggestInt
    b, w = read().parseBiggestInt
    t = read().parseBiggestInt
  if v <= w:
    echo "NO"
    return
  if (v - w) * t >= abs(a - b):
    echo "YES"
  else:
    echo "NO"
main()
