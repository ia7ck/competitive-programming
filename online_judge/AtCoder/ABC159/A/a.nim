import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let n, m = read().parseInt

  var ans = 0
  ans += n * (n - 1) div 2
  ans += m * (m - 1) div 2
  echo ans

main()
