import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  var n = read().parseInt

  var ans = 0
  ans += n div 500 * 1000
  n = n - n div 500 * 500
  ans += n div 5 * 5
  echo ans

main()
