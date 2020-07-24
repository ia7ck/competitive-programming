
import strutils, math

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc f(nn: int): seq[(int, int)] =
  var
    n = nn
    res = newSeq[(int, int)]()
  for i in 2..n:
    var e = 0
    while n mod i == 0:
      n = n div i
      e += 1
    if e > 0:
      res.add((i, e))
  return res

proc main() =
  let n = read().parseInt
  var c = newSeq[int](100)
  for i in 1..n:
    let a = f(i)
    for it in a:
      let (p, e) = it
      c[p] += e
    # p^e * p^f = p^(e+f)
  # echo c
  var d = newSeq[int](100)
  for k in 1..<d.len:
    for p in 2..<c.len:
      if c[p] == k:
        d[k] += 1
  # echo d
  let x = d[4..^1].sum
  var ans = x * (x - 1) div 2 * (d[2..^1].sum - 2)
  ans += d[24..^1].sum * (d[2..^1].sum - 1)
  ans += d[14..^1].sum * (d[4..^1].sum - 1)
  ans += d[74..^1].sum
  echo ans
main()
