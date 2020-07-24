import strutils, sequtils, math, future

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    h, w, k = read().parseInt
    c = newSeqWith(h, read())

  var ans = 123456789
  for bits in 0..<(1 shl (h - 1)):
    var map = newSeq[int](h)
    block:
      var x = 0
      for i in 0..<(h - 1):
        map[i] = x
        if ((bits shr i) and 1) == 1:
          x += 1
      map[h - 1] = x
    let n = bits.int32.countBits32 + 1 # x + 1
    var
      cost = n - 1
      ok = true
      cnt = newSeq[int](n)
    for j in 0..<w:
      var a = newSeq[int](n)
      for i in 0..<h:
        if c[i][j] == '1':
          a[map[i]] += 1
      if a.anyIt(it > k):
        ok = false
        break
      if toSeq(0..(n - 1)).any(i => cnt[i] + a[i] > k):
        cost += 1
        for i in 0..<n:
          cnt[i] = a[i]
      else:
        for i in 0..<n:
          cnt[i] += a[i]
    if ok:
      ans = min(ans, cost)
  echo ans
main()
