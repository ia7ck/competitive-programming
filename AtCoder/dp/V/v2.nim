import strutils, sequtils, queues

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let n, m = read().parseInt
  var g = newSeqWith(n, newSeq[int]())
  for i in 1..<n:
    let a, b = read().parseInt
    g[a - 1].add(b - 1)
    g[b - 1].add(a - 1)

  # 0 を根とする
  # dp[i]: 部分木 i 以下のみを考えたとき i を黒にする塗り方
  # i のそれぞれの子 j について
  #   j を白にする→部分木 j の頂点は全て白: 1 通り
  #   j を黒にする→dp[j] 通り
  var dp = newSeq[int64](n)
  proc dfs(i, p: int): int64 =
    dp[i] = 1
    for j in g[i]:
      if j != p:
        dp[i] = dp[i] * (dfs(j, i) + 1) mod m
    return dp[i]
  discard dfs(0, -1)
  var ans = newSeq[int64](n)
  var q = initQueue[(int, int, int64)]()
  q.enqueue((0, -1, 1.int64))
  while q.len > 0:
    # a: i を根としたときの dp[p] ぽい値
    let (i, p, a) = q.dequeue
    let c = g[i].len
    # le[i]: 左から i = 1, 2, ... 番目までの dp[] の積
    var le = newSeqWith(c + 1, 1.int64)
    for o in 0..<c:
      let j = g[i][o]
      le[o + 1] = le[o] * (if j == p: (a + 1) else: (dp[j] + 1)) mod m
    # ri[i]: 全部の積/le[i]  (i = 0, 1, ...)
    var ri = newSeqWith(c + 1, 1.int64)
    for o in countdown(c - 1, 0):
      let j = g[i][o]
      ri[o] = ri[o + 1] * (if j == p: (a + 1) else: (dp[j] + 1)) mod m
    for o, j in g[i]:
      if j != p:
        q.enqueue((j, i, le[o] * ri[o + 1] mod m))
    ans[i] = le[c] # = ri[0]
  echo ans.mapIt($it).join("\n")
main()
