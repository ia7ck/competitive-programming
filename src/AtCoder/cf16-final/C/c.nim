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

proc same(this: var UnionFind, i, j: int): bool =
  return this.find(i) == this.find(j)

proc main() =
  var n, m: int
  (n, m) = stdin.readLine.strip.split.map(parseInt)
  var uf = initUnionFind(n + m)
  for i in 0..<n:
    let langs = stdin.readLine.strip.split.map(parseInt)
    for j in langs[1..^1]:
      uf.unite(i, n + (j - 1))
  let ok = (1..<n).mapIt(it).allIt(uf.same(0, it))
  if ok:
    echo "YES"
  else:
    echo "NO"

main()
