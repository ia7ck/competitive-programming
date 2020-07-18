import strutils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let n, m, Q = read().parseInt
  type P = tuple[a, b, c, d: int]
  var r = newSeq[P]()
  for i in 0..<Q:
    let a, b, c, d = read().parseInt
    r.add((a - 1, b - 1, c, d))
  var ans = 0
  proc solve(v: var seq[int]) =
    if v.len == n:
      var score = 0
      for (a, b, c, d) in r:
        if v[b] - v[a] == c:
          score += d
      ans = max(ans, score)
    else:
      for x in v[^1]..m:
        v.add(x)
        solve(v)
        discard v.pop
  for x in 1..m:
    var v = @[x]
    solve(v)
  echo ans
main()
