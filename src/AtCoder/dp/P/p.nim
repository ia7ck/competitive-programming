import strutils, sequtils

let MOD = 1_000_000_000+7
var memo: seq[seq[int64]]
proc f(g: seq[seq[int]], i, p, c: int): int64 =
  if memo[i][c]>=0:
    result = memo[i][c]
  else:
    result = 1
    for j in g[i]:
      if j!=p:
        var a = f(g, j, i, 0)
        if c==0:
          a+=f(g, j, i, 1)
        result = (result*a) mod MOD
    memo[i][c] = result

proc main() =
  let n = stdin.readLine.parseInt
  var g = newSeqWith(n, newSeq[int]())
  for _ in 0..<(n-1):
    let xy = stdin.readLine.split.map(parseInt)
    g[xy[0]-1].add(xy[1]-1)
    g[xy[1]-1].add(xy[0]-1)
  memo = newSeqWith(n, newSeqWith(2, -1'i64))
  let res = (f(g, 0, -1, 0)+f(g, 0, -1, 1)) mod MOD
  echo res

main()
