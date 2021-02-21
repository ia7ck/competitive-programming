import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    n = read().parseInt
    c = newSeqWith(n, read())

  proc sym(a: seq[string]): bool =
    for i in 0..<n:
      for j in 0..<i:
        if a[i][j] != a[j][i]:
          return false
    return true
  var ans: int64 = 0
  for d in 0..<n:
    var a = newSeq[string](n)
    for i in 0..<n:
      a[(i + d) mod n] = c[i]
    if sym(a):
      ans += n
  echo ans
main()
