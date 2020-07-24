import strutils, sequtils, algorithm

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

type UnionFind = object
  par: seq[int]
  card: seq[int]

proc initUnionFind(n: int): UnionFind =
  var
    par = newSeq[int](n)
    card = newSeq[int](n)
  for i in 0..<n:
    par[i] = i
  fill(card, 1)
  return UnionFind(par: par, card: card)

proc find(this: var UnionFind, i: int): int =
  if i == this.par[i]:
    return this.par[i]
  this.par[i] = this.find(this.par[i])
  return this.par[i]

proc unite(this: var UnionFind, i, j: int) =
  var
    pi = this.find(i)
    pj = this.find(j)
  if pi == pj:
    return
  if this.card[pi] < this.card[pj]:
    swap(pi, pj)
  this.par[pj] = pi
  this.card[pi] += this.card[pj]

proc same(this: var UnionFind, i, j: int): bool =
  return this.find(i) == this.find(j)

proc size(this: var UnionFind, i: int): int =
  return this.card[this.find(i)]

proc main() =
  let n, m = read().parseInt
  var
    a = newSeq[int](m)
    b = newSeq[int](m)
  for i in 0..<m:
    (a[i], b[i]) = (read().parseInt, read().parseInt)
    a[i] -= 1
    b[i] -= 1
  var
    cc: int64 = 0
    uf = initUnionFind(n)
    ans = newSeq[int64](m)
  for i in countdown(m - 1, 0):
    ans[i] = n * (n - 1) div 2 - cc
    if not uf.same(a[i], b[i]):
      let
        sa: int64 = uf.size(a[i])
        sb: int64 = uf.size(b[i])
      cc -= sa * (sa - 1) div 2
      cc -= sb * (sb - 1) div 2
      cc += (sa + sb) * (sa + sb - 1) div 2
      uf.unite(a[i], b[i])
  echo ans.mapIt($it).join("\n")
main()
