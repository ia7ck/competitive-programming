import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

const mo: int64 = 1000000000 + 7
type mint = distinct int64
proc toMint[Int](a: Int): mint =
  var res = a.int64 mod mo
  if res < 0:
    res = res + mo
  return res.mint
proc `+`[IntA, IntB](a: IntA, b: IntB): mint = (a.int64 + b.int64).toMint
proc `-`[IntA, IntB](a: IntA, b: IntB): mint = (a.int64 - b.int64).toMint
proc `*`[IntA, IntB](a: IntA, b: IntB): mint = (a.int64 * b.int64).toMint
proc pow[Int](a: Int, x: int64): mint =
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
proc inv[Int](a: Int): mint = pow(a, mo - 2)
proc `div`[IntA, IntB](a: IntA, b: IntB): mint = a * inv(b)
proc `$`(a: mint): string {.borrow.}

template yieldCombination(n: int = 20) =
  var
    fac {.inject.} = newSeq[mint](n)
    inv {.inject.} = newSeq[mint](n)
  fac[0] = 1.toMint
  for i in 1..<n:
    fac[i] = i * fac[i - 1]
  inv[n - 1] = 1 div fac[n - 1]
  for i in countdown(n - 2, 0):
    inv[i] = inv[i + 1] * (i + 1)
  let binom {.inject.} =
    proc(a, b: int): mint =
      if a < 0 or b < 0: return 0.toMint
      if a < b: return 0.toMint
      return fac[a] * inv[b] * inv[a - b]

proc main() =
  let n = read().parseInt
  var g = newSeqWith(n, newSeq[int]())
  for i in 0..<(n - 1):
    let a, b = read().parseInt
    g[a - 1].add(b - 1)
    g[b - 1].add(a - 1)

  # 根: 頂点 1
  # c_1, c_2, ..., c_k: 頂点 1 の子
  # cnt[i]: 部分木 i に含まれる頂点数
  # sub[i]:
  #   部分木 i の中で考えて、
  #   頂点 i を左端に、残りの頂点を、親が左・子が右になるよう一列に並べる方法の数
  #   1. cnt[i] - 1 個の頂点を並べる ... fac[cnt[i] - 1] をかける
  #   2. 部分木 c_j 内の頂点を一旦まとめて考える ... fac[cnt[c_j]] で割る (j = 1, 2, ..., k)
  #   3. 部分木 c_j 内の頂点を並べる ... sub[c_j] をかける (j = 1, 2, ..., k)
  var cnt = newSeq[int](n)
  proc dfs(i, p: int): int =
    cnt[i] = 1
    for j in g[i]:
      if j != p:
        cnt[i] += dfs(j, i)
    return cnt[i]
  discard dfs(0, -1)
  yieldCombination(n + 1)
  var sub = newSeqWith(n, 1.toMint)
  proc sfd(i, p: int): mint =
    sub[i] = fac[cnt[i] - 1]
    for j in g[i]:
      if j != p:
        sub[i] = sub[i] * sfd(j, i) * fac[cnt[j]].inv
    return sub[i]
  discard sfd(0, -1)
  var ans = newSeq[mint](n)
  ans[0] = sub[0]
  # a: i を根としたときの sub[p] みたいな感じ
  proc solve(i, p: int, a: mint) =
    var
      d = 1.toMint
      s = 1.toMint
    for j in g[i]:
      if j != p:
        d = d * fac[cnt[j]]
        s = s * sub[j]
    # fac[cnt[0] - cnt[i]]: i を根としたときの部分木 p に含まれる頂点数
    if p >= 0:
      ans[i] = fac[n - 1] * d.inv * fac[cnt[0] - cnt[i]].inv * s * a
    for j in g[i]:
      if j != p:
        # ???
        solve(j, i, fac[cnt[0] - cnt[j] - 1] * d.inv * fac[cnt[0] - cnt[i]].inv * fac[cnt[j]] * s * sub[j].inv * a)
  solve(0, -1, 1.toMint)
  echo ans.mapIt($it).join("\n")
main()
