import strutils, algorithm, math

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let D, G = read().parseInt
  var
    p = newSeq[int](D)
    c = newSeq[int](D)
  for i in 0..<D:
    (p[i], c[i]) = (read().parseInt, read().parseInt)
  var ans = p.sum
  for bits in 0..<(1 shl D):
    var
      q = 0
      g = 0
      a = newSeq[int]()
    for i in 0..<D:
      if ((bits shr i) and 1) == 1:
        q += p[i]
        g += 100 * (i + 1) * p[i] + c[i]
      else:
        for j in 0..<p[i]:
          a.add(100 * (i + 1))
    a.sort(cmp)
    a.reverse
    for it in a:
      if g < G:
        g += it
        q += 1
    ans = min(ans, q)
  echo ans
main()
