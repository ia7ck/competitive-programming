import strutils, sequtils

proc dfs(g: seq[seq[int]], cur, par: int, acc: int64, cnt: var seq[int64]) =
  let orig = cnt[cur]
  cnt[cur] += acc
  for nxt in g[cur]:
    if nxt != par:
      dfs(g, nxt, cur, acc + orig, cnt)

proc main() =
  var n, q: int
  (n, q) = stdin.readLine.strip.split.map(parseInt)
  var g = newSeqWith(n, newSeq[int]())
  for i in 1..<n:
    var a, b: int
    (a, b) = stdin.readLine.strip.split.map(parseInt)
    a -= 1
    b -= 1
    g[a].add(b)
    g[b].add(a)
  var cnt = newSeq[int64](n)
  for i in 0..<q:
    var p, x: int
    (p, x) = stdin.readLine.strip.split.map(parseInt)
    cnt[p - 1] += x
  dfs(g, 0, -1, 0, cnt)
  echo cnt.mapIt($it).join(" ")
main()
