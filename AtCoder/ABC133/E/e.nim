import strutils, sequtils

const mo: int64 = 1000000007
var g: seq[seq[int]]
var n, k: int
proc dfs(i, p: int): int64 =
  var c = if p == -1: k - 1 else: k - 2
  var res: int64 = 1
  for j in g[i]:
    if j == p: continue
    res = (res * c mod mo) * dfs(j, i) mod mo
    c -= 1
  return res

proc main() =
  (n, k) = stdin.readLine.strip.split.map(parseInt)
  g = newSeqWith(n, newSeq[int]())
  for i in 1..<n:
    var a, b: int
    (a, b) = stdin.readLine.strip.split.map(parseInt)
    a -= 1
    b -= 1
    g[a].add(b)
    g[b].add(a)

  echo (k * dfs(0, -1) mod mo)
main()
