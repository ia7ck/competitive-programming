import strutils, sequtils, deques, math

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let n = read().parseInt
  type E = tuple[to, cost: int]
  var g = newSeqWith(n, newSeq[E]())
  for i in 1..<n:
    let a, b, c = read().parseInt
    g[a - 1].add((b - 1, c))
    g[b - 1].add((a - 1, c))
  # 0 を根とする
  # size[i]: 部分木 i に含まれる頂点数
  # dp[i]: Σd(i, i の子孫)
  var
    size = newSeq[int](n)
    dp = newSeq[int](n)
  proc dfs(i, p: int) =
    size[i] = 1
    for e in g[i]:
      if e.to != p:
        dfs(e.to, i)
        size[i] += size[e.to]
        dp[i] += dp[e.to] + size[e.to] * e.cost
  dfs(0, -1)
  # echo dp
  var ans = newSeq[int](n)
  var q = initDeque[(int, int, int, int)]()
  q.addLast((0, -1, 0, 0))
  while q.len > 0:
    let (i, p, a, b) = q.popFirst
    var tot = 0
    for e in g[i]:
      if e.to == p:
        tot += b + a * e.cost
      else:
        tot += dp[e.to] + size[e.to] * e.cost
    ans[i] = tot
    for e in g[i]:
      if e.to != p:
        let
          na = a + size[i] - size[e.to]
          nb = tot - dp[e.to] - size[e.to] * e.cost
        q.addLast((e.to, i, na, nb))
  echo ans.sum
main()
