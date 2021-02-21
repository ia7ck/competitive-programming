import strutils, sequtils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  var n, s, t = read().parseInt
  var g = newSeqWith(n, newSeq[int]())
  for i in 1..<n:
    let a, b = read().parseInt
    g[a - 1].add(b - 1)
    g[b - 1].add(a - 1)
  var
    h = newSeq[int](n)
    dep = newSeq[int](n)
    par = newSeq[int](n)
  proc dfs(i, p: int): int =
    par[i] = p
    for j in g[i]:
      if j != p:
        dep[j] = dep[i] + 1
        h[i] = max(h[i], dfs(j, i) + 1)
    return h[i]
  s -= 1
  t -= 1
  discard dfs(t, -1)
  var
    d = 0
    ans = 0
  while s != t:
    if d < dep[s]:
      ans = max(ans, dep[s] + h[s] - 1)
    s = par[s]
    d += 1
  echo ans
main()
