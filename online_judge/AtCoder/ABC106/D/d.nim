import strutils, sequtils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let n, m, Q = read().parseInt
  var a = newSeqWith(n + 1, newSeq[int](n + 1))
  for i in 0..<m:
    let le, ri = read().parseInt
    a[le][ri] += 1
  for i in 0..n:
    for j in 0..<n:
      a[i][j + 1] += a[i][j]
  for q in 0..<Q:
    let x, y = read().parseInt
    var ans = 0
    for i in x..y:
      ans += a[i][y] - a[i][i - 1]
    echo ans
main()
