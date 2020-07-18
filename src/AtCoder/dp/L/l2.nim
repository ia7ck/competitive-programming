import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

var
  a: seq[int64]
  memo: seq[seq[int64]]
const inf = int64.high

proc solve(i, j, t: int): int64 =
  if memo[i][j] != inf:
    return memo[i][j]
  if i == j:
    return a[i] * t
  let
    fr = solve(i + 1, j, t * (-1)) + a[i] * t
    bk = solve(i, j - 1, t * (-1)) + a[j] * t
  if t == 1:
    memo[i][j] = max(fr, bk)
  else:
    memo[i][j] = min(fr, bk)
  return memo[i][j]

proc main() =
  let n = read().parseInt
  a = newSeqWith(n, read().parseBiggestInt)

  memo = newSeqWith(n, newSeqWith(n, inf))
  echo solve(0, n - 1, 1)
main()
