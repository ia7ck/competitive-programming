import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

var
  n: int
  a: seq[int64]
  cul: seq[int64]
  memo: seq[seq[int64]]

proc solve(i, j: int): int64 =
  if memo[i][j] >= 0:
    return memo[i][j]
  if i == j:
    return 0
  var res = int64.high
  for t in i..<j:
    res = min(res, solve(i, t) + solve(t + 1, j) + cul[j + 1] - cul[i])
  memo[i][j] = res
  return res

proc main() =
  n = read().parseInt
  a = newSeqWith(n, read().parseBiggestInt)

  cul = newSeq[int64](n + 1)
  for i in 0..<n:
    cul[i + 1] = cul[i] + a[i]
  memo = newSeqWith(n, newSeqWith(n, -1.int64))
  echo solve(0, n - 1)
main()
