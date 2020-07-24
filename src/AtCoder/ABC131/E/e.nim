import strutils, sequtils

proc main() =
  var n, k: int
  (n, k) = stdin.readLine.strip.split.map(parseInt)

  var t = (n - 1) * (n - 2) div 2
  if k > t:
    echo "-1"
    return

  type E = tuple[u: int, v: int]
  var edges = newSeq[E]()
  for j in 1..<n:
    edges.add((0, j))
  # 条件を満たす頂点対が t 個
  for i in 1..<n:
    for j in (i + 1)..<n:
      if t > k:
        edges.add((i, j))
        t -= 1
  echo edges.len
  echo edges.mapIt($(it.u + 1) & " " & $(it.v + 1)).join("\n")
main()
