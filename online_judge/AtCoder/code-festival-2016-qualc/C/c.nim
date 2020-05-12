import strutils, sequtils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let
    n = read().parseInt
    t = newSeqWith(n, read().parseBiggestInt)
    a = newSeqWith(n, read().parseBiggestInt)

  var u = newSeq[bool](n)
  u[0] = true
  for i in 1..<n:
    u[i] = t[i - 1] < t[i]
  var b = newSeq[bool](n)
  b[^1] = true
  for i in countdown(n - 2, 0):
    b[i] = a[i] > a[i + 1]
  var ans: int64 = 1
  const mo: int64 = 1000000000 + 7
  for i in 0..<n:
    if u[i]:
      if b[i]:
        if t[i] != a[i]:
          ans = 0
      else:
        if t[i] > a[i]:
          ans = 0
    else:
      if b[i]:
        if a[i] > t[i]:
          ans = 0
      else:
        ans = ans * min(t[i], a[i]) mod mo
  echo ans
main()
