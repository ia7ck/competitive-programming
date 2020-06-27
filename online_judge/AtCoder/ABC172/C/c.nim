import strutils, sequtils, algorithm

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    n, m = read().parseInt
    k = read().parseBiggestInt
  var
    a = newSeqWith(n, read().parseBiggestInt)
    b = newSeqWith(m, read().parseBiggestInt)
  for i in 1..<n:
    a[i] += a[i - 1]
  for i in 1..<m:
    b[i] += b[i - 1]

  var ans = 0
  for i in 0..n: # i 冊読んだ
    var s: int64 = 0
    if i >= 1:
      s = a[i - 1]
    if s <= k:
      let j = b.upperBound(k - s)
      # echo i, " ", j
      ans = max(ans, i + j)
  echo ans
main()
