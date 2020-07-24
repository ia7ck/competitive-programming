import strutils, algorithm, sequtils

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

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let n, m, s = read().parseInt
  var g = newSeqWith(n, newSeq[int]())
  for i in 0..<m:
    let a, b = read().parseInt
    g[a - 1].add(b - 1)
    g[b - 1].add(a - 1)
  var
    uf = initUnionFind(n)
    ans = newSeq[int]()
  for i in countdown(n - 1, 0):
    for j in g[i]:
      if j > i:
        uf.unite(i, j)
    if uf.same(i, s - 1):
      ans.add(i)
  ans.reverse
  echo ans.mapIt($(it + 1)).join("\n")
main()
