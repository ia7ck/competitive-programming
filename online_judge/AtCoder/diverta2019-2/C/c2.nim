import strutils, sequtils, algorithm, math

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let n = read().parseInt
  var a = newSeqWith(n, read().parseBiggestInt)

  a.sort(cmp)
  a.reverse
  var
    mx = int64.low
    ans = newSeq[(int64, int64)]()
  if a[^1] >= 0:
    mx = a[0..<(n - 1)].sum - a[^1]
    var x = a[^1]
    for i in 1..<(n - 1):
      ans.add((x, a[i]))
      x -= a[i]
    ans.add((a[0], x))
  elif a[0] <= 0:
    mx = a[0] + a[1..^1].sum * (-1)
    var x = a[0]
    for i in 1..<n:
      ans.add((x, a[i]))
      x -= a[i]
  else:
    let k = a.filterIt(it > 0).len - 1
    mx = a.mapIt(abs(it)).sum
    var x = a[k + 1]
    for i in 1..k:
      ans.add((x, a[i]))
      x -= a[i]
    ans.add((a[0], x))
    x = a[0] - x
    for i in (k + 2)..<n:
      ans.add((x, a[i]))
      x -= a[i]
  echo mx
  for it in ans:
    let (a, b) = it
    echo a, " ", b
main()
