import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    n = read().parseInt
    a = newSeqWith(n, read().parseInt)
  const m = 20
  var
    cnt = newSeq[int](m)
    le = 0
    ans: int64 = 0
  for ri in 0..<n:
    for i in 0..<m:
      cnt[i] += (a[ri] shr i) and 1
    while cnt.anyIt(it > 1):
      for i in 0..<m:
        cnt[i] -= (a[le] shr i) and 1
      le += 1
    ans += ri - le + 1
  echo ans
main()
