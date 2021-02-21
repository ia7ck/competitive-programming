import strutils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let
    a, b, n = read().parseBiggestInt
    k = max(1, (n + 1) div b)
    x = min(n, k * b - 1)
  var ans = a * x div b - a * (x div b)
  echo ans

main()
