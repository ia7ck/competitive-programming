import strutils, sequtils, algorithm

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let
    n, A, B, C = read().parseInt
    p = newSeqWith(n, read().parseInt)
  var ans = int.high
  proc dfs(i, s, a, b, c: int) =
    if i == n:
      let x = @[a, b, c].sortedByIt(-it)
      if x.anyIt(it == 0):
        return
      ans = min(
        ans,
        s + abs(A - x[0]) + abs(B - x[1]) + abs(C - x[2]))
    else:
      if a > 0:
        dfs(i + 1, s + 10, a + p[i], b, c)
      else:
        dfs(i + 1, s, a + p[i], b, c)
      if b > 0:
        dfs(i + 1, s + 10, a, b + p[i], c)
      else:
        dfs(i + 1, s, a, b + p[i], c)
      if c > 0:
        dfs(i + 1, s + 10, a, b, c + p[i])
      else:
        dfs(i + 1, s, a, b, c + p[i])
      dfs(i + 1, s, a, b, c)
  dfs(0, 0, 0, 0, 0)
  echo ans
main()
