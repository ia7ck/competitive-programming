import strutils, sequtils, math, algorithm

type FenwickTree = object
  n: int
  dat: seq[int]

proc initFenwickTree(n: int): FenwickTree =
  result.n = nextPowerOfTwo(n)
  result.dat = newSeq[int](result.n + 1)

proc add(this: var FenwickTree, i, x: int) =
  var j = i
  while j <= this.n:
    this.dat[j] += x
    j += (j and (-j))         # bitwise and

proc sum(this: var FenwickTree, r: int): int =
  var j = r
  while j > 0:
    result += this.dat[j]
    j -= (j and (-j))

proc sum(this: var FenwickTree, left, right: int): int =
  if right < 0 or left > right:
    return 0
  return this.sum(right) - this.sum(left - 1)

# ##########################################################

var
  g: seq[seq[int]]
  ft: FenwickTree
  less_cnt: seq[int]
  par_less_cnt: seq[int]
  ans: seq[int]

proc dfs(i, p: int) =
  less_cnt[i] = -ft.sum(1, i)
  for j in g[i]:
    if j == p: continue
    par_less_cnt[j] = -ft.sum(1, i)
    dfs(j, i)
    par_less_cnt[j] += ft.sum(1, i) # 部分木 j の中で i 未満の個数
  less_cnt[i] += ft.sum(1, i) # 部分木 i の中で i 未満の個数
  ft.add(i + 1, 1)


proc sfd(i, p, acc: int) =
  ans[i] = acc
  for j in g[i]:
    if j == p: continue
    sfd(j, i, acc + (j - less_cnt[j]) - par_less_cnt[j])

proc main() =
  let n = stdin.readLine.strip.parseInt
  g = newSeqWith(n, newSeq[int]())
  for i in 1..<n:
    var a, b: int
    (a, b) = stdin.readLine.strip.split.map(parseInt)
    g[a - 1].add(b - 1)
    g[b - 1].add(a - 1)

  ft = initFenwickTree(n)
  less_cnt = newSeq[int](n)
  par_less_cnt = newSeq[int](n)
  dfs(0, -1)

  ft.dat.fill(0)
  ans = newSeq[int](n)
  sfd(0, -1, less_cnt.sum)
  echo ans.mapIt($it).join("\n")
main()
