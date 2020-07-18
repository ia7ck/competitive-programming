import strutils, sequtils, algorithm

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let n = read().parseInt
  var d = newSeqWith(n, read().parseInt)

  doAssert(n <= 3000)

  d.sort(cmp)
  var ans: int64 = 0
  const mo: int64 = 1000000000 + 7
  for q in d:
    for r in d:
      if q * 2 <= r:
        let
          a: int64 = d.upperBound(q div 2)
          b: int64 = n - d.lowerBound(r * 2)
        # echo a, " ", b
        ans = (ans + a * b mod mo) mod mo
  echo ans
main()
