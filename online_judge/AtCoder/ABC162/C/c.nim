import strutils, math

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let k = read().parseInt

  var ans: int64 = 0
  for a in 1..k:
    for b in 1..k:
      for c in 1..k:
        ans += gcd(a, gcd(b, c))
  echo ans

main()
