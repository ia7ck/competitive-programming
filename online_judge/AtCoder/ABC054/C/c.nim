import strutils, sequtils, algorithm

proc main() =
  var n, m: int
  (n, m) = stdin.readLine.strip.split.map(parseInt)
  var g = newSeqWith(n, newSeq[bool](n))
  for i in 0..<m:
    var a, b: int
    (a, b) = stdin.readLine.strip.split.map(parseInt)
    g[a - 1][b - 1] = true
    g[b - 1][a - 1] = true

  var
    vs = toSeq(1..n).mapIt(it - 1)
    ans = 0
  while true:
    var ok = true
    for i in 1..<n:
      if not g[vs[i - 1]][vs[i]]:
        ok = false
    if vs[0] == 0 and ok:
      ans += 1
    if not vs.nextPermutation:
      break
  echo ans
main()
