import strutils, sequtils

type UnionFind = object
  n: int
  par: seq[int]
  size: seq[int]

proc initUnionFind(n: int): UnionFind =
  result.n = n
  result.par = (0..<n).mapIt(it)
  result.size = newSeqWith(n, 1)

proc find(this: var UnionFind, i: int): int =
  if this.par[i] != i:
    this.par[i] = this.find(this.par[i])
  return this.par[i]

proc unite(this: var UnionFind, i, j: int) =
  var
    pi = this.find(i)
    pj = this.find(j)
  if pi == pj:
    return
  if this.size[pi] < this.size[pj]:
    swap(pi, pj)
  this.par[pj] = pi
  this.size[pi] += this.size[pj]

proc same(this: var UnionFind, i, j: int): bool =
  return this.find(i) == this.find(j)

proc componentSize(this: var UnionFind, i: int): int =
  return this.size[this.find(i)]

proc main() =
  var n, m, k: int
  (n, m, k) = stdin.readLine.strip.split.map(parseInt)
  var
    g = newSeqWith(n, newSeq[int]())
    h = newSeqWith(n, newSeq[int]())
    uf = initUnionFind(n)
  for i in 0..<m:
    var a, b: int
    (a, b) = stdin.readLine.strip.split.map(parseInt)
    g[a - 1].add(b - 1)
    g[b - 1].add(a - 1)
    uf.unite(a - 1, b - 1)
  for i in 0..<k:
    var a, b: int
    (a, b) = stdin.readLine.strip.split.map(parseInt)
    h[a - 1].add(b - 1)
    h[b - 1].add(a - 1)

  var ans = newSeq[int]()
  for i in 0..<n:
    var cand = uf.componentSize(i)
    cand -= 1 # 自分
    for j in g[i]:
      if uf.same(i, j):
        cand -= 1 # 直接の友達
    for j in h[i]:
      if uf.same(i, j):
        cand -= 1 # ブロック
    ans.add(cand)
  echo ans.mapIt($it).join(" ")
main()
