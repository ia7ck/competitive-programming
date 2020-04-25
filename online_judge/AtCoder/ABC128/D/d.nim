import strutils, math, sequtils, algorithm

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let
    n, k = read().parseInt
    a = newSeqWith(n, read().parseInt)

  var ans = int.low
  for i in 0..<n:
    # [0, i)
    for j in i..n:
      # [j, n)
      # -10, 8, 2, 1, 2, 6
      if i + (n - j) <= k:
        let
          b = (a[0..<i] & a[j..^1]).sortedByIt(it)
          s = b.sum
        # echo i, " ", j, " ", b, " ", s
        for p in 0..(k - i - (n - j)):
          ans = max(ans, s - b[0..<min(b.len, p)].sum)
  echo ans
main()
