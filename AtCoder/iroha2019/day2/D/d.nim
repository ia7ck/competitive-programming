import strutils, sequtils, algorithm

type UnionFind = object
  n: int
  par: seq[int]
  card: seq[int]

proc initUnionFind(sz: int): UnionFind =
  var
    par = (0..<sz).mapIt(it)
    card = newSeqWith(sz, 1)
  result = UnionFind(n: sz, par: par, card: card)

proc find(this: var UnionFind, i: int): int =
  if i != this.par[i]:
    this.par[i] = find(this, this.par[i])
  return this.par[i]

proc unite(this: var UnionFind, i, j: int) =
  var
    ri = this.find(i)
    rj = this.find(j)
  if ri == rj:
    return
  else:
    if this.card[ri] < this.card[rj]:
      swap(ri, rj)
    this.par[rj] = ri
    this.card[ri] += this.card[rj]

proc same(this: var UnionFind, i, j: int): bool =
  return this.find(i) == this.find(j)

proc main() =
  var n, m: int
  (n, m) = stdin.readLine.strip.split.map(parseInt)
  type Edge = object
    a, b: int
    c: int64
    idx: int
  var edges = newSeq[Edge](m)
  for i in 0..<m:
    var a, b, c: int
    (a, b, c) = stdin.readLine.strip.split.map(parseInt)
    edges[i] = Edge(a: a - 1, b: b - 1, c: -c, idx: i)
  edges.sort(proc(x, y: Edge): int = cmp(x.c, y.c))
  var
    uf = initUnionFind(n)
    ans = newSeq[int]()
  for e in edges:
    if not uf.same(e.a, e.b):
      uf.unite(e.a, e.b)
      ans.add(e.idx + 1)
  ans.sort(system.cmp)
  echo ans.mapIt($it).join("\n")
main()
