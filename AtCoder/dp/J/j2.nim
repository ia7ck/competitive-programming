import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

var
  n: int
  memo: seq[seq[seq[float64]]]
proc solve(c0, c1, c2, c3: int): float64 =
  if memo[c1][c2][c3] >= 0:
    return memo[c1][c2][c3]
  let s = c1 + c2 + c3
  if s == 0:
    return 0
  var res = 1.0
  if c1 >= 1:
    res += solve(c0 + 1, c1 - 1, c2, c3) * (c1 / n)
  if c2 >= 1:
    res += solve(c0, c1 + 1, c2 - 1, c3) * (c2 / n)
  if c3 >= 1:
    res += solve(c0, c1, c2 + 1, c3 - 1) * (c3 / n)
  res = res * (n / s)
  memo[c1][c2][c3] = res
  return res

proc main() =
  n = read().parseInt
  let a = newSeqWith(n, read().parseInt)
  var c = newSeq[int](4)
  for it in a:
    c[it] += 1
  memo = newSeqWith(n + 1, newSeqWith(n + 1, newSeqWith(n + 1, -1.0)))
  let ans = solve(0, c[1], c[2], c[3])
  echo ans.formatFloat
main()
