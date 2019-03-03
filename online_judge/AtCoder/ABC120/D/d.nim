import strutils, sequtils, algorithm, future

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

proc unite(this: var UnionFind, i, j: int): bool =
  var
    ri = this.find(i)
    rj = this.find(j)
  if ri == rj:
    return false
  else:
    if this.card[ri] < this.card[rj]:
      swap(ri, rj)
    this.par[rj] = ri
    this.card[ri] += this.card[rj]
    return true

proc size(this: var UnionFind, i: int): int =
  return this.card[find(this, i)]

proc main() =
  let
    nm = stdin.readLine.strip.split.map(parseInt)
    (n, m) = (nm[0], nm[1])
  var abi = (0..<m).mapIt(stdin.readLine.split.map(parseInt).map(it => it-1))

  abi.reverse
  var
    uf = initUnionFind(n)
    ans = newSeq[int64](m)
  ans[0] = (int64)n * (n-1) div 2
  for i, ab in abi[0..<(m-1)]:
    ans[i+1] = ans[i]
    let
      x: int64 = uf.size(ab[0])
      y: int64 = uf.size(ab[1])
    if uf.unite(ab[0], ab[1]):
      ans[i+1] += (x*(x-1) div 2) + (y*(y-1) div 2)
      let z: int64 = uf.size(ab[0])
      ans[i+1] -= z*(z-1) div 2
  ans.reverse
  for a in ans:
    echo a

main()
