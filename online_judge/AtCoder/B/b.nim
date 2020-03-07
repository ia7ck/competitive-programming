import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let n, a, b = read().parseBiggestInt

  let c = n div (a + b)
  var ans = c * a
  ans += min(n - c * (a + b), a)
  echo ans

main()
