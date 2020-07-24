import strutils, sequtils

proc dfs(i: int, g: seq[seq[int]], seen: var seq[bool]) =
  seen[i] = true
  for j in g[i]:
    if not seen[j]:
      dfs(j, g, seen)

proc main() =
  var n, m: int
  (n, m) = stdin.readLine.strip.split.map(parseInt)
  var g = newSeqWith(n, newSeq[int]())
  for i in 0..<m:
    var x, y, z: int
    (x, y, z) = stdin.readLine.strip.split.map(parseInt)
    x -= 1
    y -= 1
    g[x].add(y)
    g[y].add(x)
  var
    seen = newSeq[bool](n)
    cnt = 0
  for i in 0..<n:
    if not seen[i]:
      dfs(i, g, seen)
      cnt += 1
  echo cnt
main()
