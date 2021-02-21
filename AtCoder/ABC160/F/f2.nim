import strutils, sequtils, queues

const mo: int64 = 1000000000 + 7
type mint = distinct int64
proc toMint[Int](a: Int): mint =
  var res = a.int64 mod mo
  if res < 0:
    res = res + mo
  return res.mint
proc `+`(a, b: mint): mint = (a.int64 + b.int64).toMint
proc `-`(a, b: mint): mint = (a.int64 - b.int64).toMint
proc `*`(a, b: mint): mint = (a.int64 * b.int64).toMint
proc `*=`[Int](a: var mint, b: Int) = a = a * b.toMint
proc pow(a: mint, x: int64): mint =
  assert(x >= 0)
  var
    res = 1.toMint
    base = a
    exp = x
  while exp > 0:
    if (exp and 1) == 1:
      res = res * base
    base = base * base
    exp = exp div 2
  return res
proc inv(a: mint): mint = pow(a, mo - 2)
proc `div`(a, b: mint): mint = a * inv(b)
proc `$`(a: mint): string {.borrow.}

template yieldCombination(n: int = 20) =
  var
    fac {.inject.} = newSeq[mint](n)
    inv {.inject.} = newSeq[mint](n)
  fac[0] = 1.toMint
  for i in 1..<n:
    fac[i] = i.toMint * fac[i - 1]
  inv[n - 1] = 1.toMint div fac[n - 1]
  for i in countdown(n - 2, 0):
    inv[i] = inv[i + 1] * (i + 1).toMint
  let binom {.inject.} =
    proc(a, b: int): mint =
      if a < 0 or b < 0: return 0.toMint
      if a < b: return 0.toMint
      return fac[a] * inv[b] * inv[a - b]

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let n = read().parseInt
  var g = newSeqWith(n, newSeq[int]())
  for i in 1..<n:
    let a, b = read().parseInt
    g[a - 1].add(b - 1)
    g[b - 1].add(a - 1)

  yieldCombination(n + 1)
  # 頂点 1 を根とする
  # size[i]: 部分木 i に含まれる頂点数
  # dp[i]: 部分木 i 以下で考えたときの書き方の数
  #   = size[i] 個の頂点を親が左・子が右になる並べ方の数
  #     (左から 1, 2, ... と書き込む)
  #   i を一番左にする
  #   i のそれぞれの子 j について
  #     size[j] 個ずつ選んで合計 size[i] - 1 個選ぶ: 多項係数
  #   選んだ size[j] 個の並べ方は dp[j] 通り
  var
    size = newSeq[int](n)
    dp = newSeq[mint](n)
  proc dfs(i, p: int) =
    size[i] = 1
    dp[i] = 1.toMint
    for j in g[i]:
      if j != p:
        dfs(j, i)
        size[i] += size[j]
        dp[i] *= dp[j] div fac[size[j]]
    dp[i] *= fac[size[i] - 1]
  dfs(0, -1)
  # echo dp
  var ans = newSeq[mint](n)
  var q = initQueue[(int, int, int, mint)]()
  q.enqueue((0, -1, 0, 1.toMint))
  while q.len > 0:
    let (i, p, a, b) = q.dequeue
    var tot = 1.toMint
    for j in g[i]:
      tot *= (if j == p: b div fac[a] else: dp[j] div fac[size[j]])
    ans[i] = tot * fac[n - 1]
    for j in g[i]:
      if j != p:
        let na = a + size[i] - size[j]
        q.enqueue((j, i, na, tot div (dp[j] div fac[size[j]]) * fac[na - 1]))
  echo ans.mapIt($it).join("\n")
main()
