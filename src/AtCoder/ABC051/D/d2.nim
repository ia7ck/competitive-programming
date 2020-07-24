import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let n, m = read().parseInt
  type E = tuple[a, b, c: int]
  var edges = newSeq[E]()
  for i in 0..<m:
    let a, b, c = read().parseInt
    edges.add((a - 1, b - 1, c))

  var d = newSeqWith(n, newSeq[int](n))
  const inf = int.high div 2
  for i in 0..<n:
    for j in 0..<n:
      if i != j:
        d[i][j] = inf
  for e in edges:
    d[e.a][e.b] = e.c
    d[e.b][e.a] = e.c
  for k in 0..<n:
    for i in 0..<n:
      for j in 0..<n:
        d[i][j] = min(d[i][j], d[i][k] + d[k][j])
  var ans = 0
  for e in edges:
    var used = false
    for i in 0..<n:
      for j in 0..<n:
        if d[i][e.a] + e.c + d[e.b][j] == d[i][j]:
          used = true
    if not used:
      ans += 1
  echo ans
main()
