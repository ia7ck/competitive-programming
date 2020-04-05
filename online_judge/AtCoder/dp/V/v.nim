import strutils, sequtils, algorithm, queues

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
  # 頂点 0 を根とする木で, 頂点 i を黒にして
  # 部分木 i 以下のみを考えたときの塗り方: dp[i]
  if n == 1:
    echo 1
    return
  var dp = newSeq[int64](n)
  proc dfs(i, p: int): int64 =
    dp[i] = 1
    for j in g[i]:
      if j == p:
        continue
      dp[i] = dp[i] * (dfs(j, i) + 1) mod m
    return dp[i]
  discard dfs(0, -1)
  # echo dp

  var ans = newSeq[int64](n)
  var q = initQueue[(int, int, int64)]()
  q.enqueue((0, -1, 1.int64))
  while q.len > 0:
    let (i, p, a) = q.dequeue
    ans[i] = a mod m # p 側の塗り方
    for j in g[i]:
      if j == p:
        continue
      ans[i] = ans[i] * (dp[j] + 1) mod m
    var cul = newSeq[int64](g[i].len)
    fill(cul, 1)
    for o in 0..<g[i].len:
      if o > 0:
        cul[o] = cul[o - 1]
      let j = g[i][o]
      if j == p:
        continue
      cul[o] = cul[o] * (dp[j] + 1) mod m
    var luc = newSeq[int64](g[i].len)
    fill(luc, 1)
    for o in countdown(g[i].len - 1, 0):
      if o + 1 < g[i].len:
        luc[o] = luc[o + 1]
      let j = g[i][o]
      if j == p:
        continue
      luc[o] = luc[o] * (dp[j] + 1) mod m
    for o in 0..<g[i].len:
      let j = g[i][o]
      if j == p:
        continue
      var b = a
      if o - 1 >= 0:
        b = b * cul[o - 1] mod m
      if o + 1 < g[i].len:
        b = b * luc[o + 1] mod m
      q.enqueue((j, i, b + 1))

  echo ans.mapIt($it).join("\n")
main()
