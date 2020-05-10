import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let n, m, x = read().parseInt
  var
    c = newSeq[int](n)
    a = newSeqWith(n, newSeq[int]())
  for i in 0..<n:
    c[i] = read().parseInt
    a[i] = newSeqWith(m, read().parseInt)
  var ans = int.high
  for bits in 0..<(1 shl n):
    var
      s = newSeq[int](m)
      cost = 0
    for i in 0..<n:
      if ((bits shr i) and 1) == 1:
        cost += c[i]
        for j in 0..<m:
          s[j] += a[i][j]
    if s.allIt(it >= x):
      ans = min(ans, cost)
  if ans == int.high:
    ans = -1
  echo ans
main()
