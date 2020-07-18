import strutils, sequtils

proc main() =
  var n, m: int
  (n, m) = stdin.readLine.strip.split.map(parseInt)
  var acc = newSeq[int](n + 1)
  for i in 0..<m:
    var a, b: int
    (a, b) = stdin.readLine.strip.split.map(parseInt)
    acc[a - 1] += 1
    acc[b] -= 1
  for i in 1..n:
    acc[i] += acc[i - 1]
  var cnt = 0
  for i in 0..<n:
    if acc[i] == m:
      cnt += 1
  echo cnt
main()
