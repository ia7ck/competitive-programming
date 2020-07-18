import strutils, sequtils, algorithm, math

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let n = read().parseInt
  var
    a = newSeqWith(n, read().parseInt)
    b = newSeqWith(n, read().parseInt)
  
  var
    ans = 0
    p2 = 1 shl 28
  for k in countdown(28, 0):
    a = a.mapIt(it and (p2 * 2 - 1))
    b = b.mapIt(it and (p2 * 2 - 1))
    b.sort(cmp)
    var cnt = 0
    for it in a:
      cnt += b.lowerBound(p2 * 2 - it) - b.lowerBound(p2 - it)
      cnt += b.lowerBound(p2 * 4 - it) - b.lowerBound(p2 * 3 - it)
    if cnt mod 2 == 1:
      ans += p2
    p2 = p2 shr 1
  echo ans
main()
