import strutils, sequtils

type E = tuple[a, b: int]
proc check(n: int, edges: seq[E]) =
  var s = newSeq[int](n)
  for e in edges:
    s[e.a - 1] += e.b
    s[e.b - 1] += e.a
  doAssert s.allIt(it == s[0])

proc main() =
  let n = stdin.readLine.strip.parseInt
  var edges = newSeq[E]()
  let a = if n mod 2 == 0: n + 1 else: n
  for i in 1..n:
    for j in 1..<i:
      if i + j == a: continue
      edges.add((i, j))
  check(n, edges)
  echo edges.len
  for e in edges:
    echo e.a, " ", e.b
main()
