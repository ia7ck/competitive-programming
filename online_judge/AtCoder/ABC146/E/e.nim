import strutils, sequtils, tables, algorithm

proc main() =
  var n, k: int
  (n, k) = stdin.readLine.strip.split.map(parseInt)
  let a = stdin.readLine.strip.split.map(parseInt)

  var b = newSeq[int](n + 1)
  for i in 1..n:
    b[i] = b[i - 1] + a[i - 1]
  for i in 1..n:
    b[i] = (b[i] - i) mod k
  var c = b
  c.sort(system.cmp)
  var o = initTable[int, int]()
  for it in c:
    if o.hasKey(it):
      discard
    else:
      o[it] = o.len
  var poss = newSeqWith(o.len, newSeq[int]())
  poss[0].add(0)
  var ans: int64 = 0
  for i in 1..n:
    let
      v = o[b[i]]
      m = poss[v].lowerBound(i - k + 1)
    ans += poss[v].len - m
    poss[v].add(i)
  echo ans
main()
