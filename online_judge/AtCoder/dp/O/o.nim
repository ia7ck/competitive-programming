import strutils, sequtils, math, queues

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

const mo: int = 1000000000 + 7
var
  n: int
  a: seq[seq[int]]
  memo: seq[int]

proc solve(bits: int): int =
  if memo[bits] >= 0:
    return memo[bits]
  if bits == (1 shl n) - 1:
    return 1
  let i = n - bits.int32.countBits32 - 1
  var res = 0
  for j in 0..<n:
    if ((bits shr j) and 1) == 0 and a[i][j] == 1:
      res = (res + solve(bits xor (1 shl j))) mod mo
  memo[bits] = res
  return res

proc main() =
  n = read().parseInt
  a = newSeqWith(n, newSeqWith(n, read().parseInt))

  memo = newSeqWith((1 shl n), -1)
  echo solve(0)

main()
