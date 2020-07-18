import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    n = read().parseInt
    a = newSeqWith(n, read().parseInt)
  proc check(b: seq[int]) =
    var tot = 0
    for x in b:
      tot = tot xor x
    for i in 0..<n:
      doAssert(a[i] == (tot xor b[i]))
  var ans = newSeq[int](n)
  for i in 0..<32:
    var cnt = 0
    for x in a:
      if ((x shr i) and 1) == 1:
        cnt += 1
    for j in 0..<n:
      if ((a[j] shr i) and 1) == 1:
        ans[j] = ans[j] xor (1 shl i)
      if cnt mod 2 == 1:
        ans[j] = ans[j] xor (1 shl i)
  check(ans)
  echo ans.join(" ")
main()
