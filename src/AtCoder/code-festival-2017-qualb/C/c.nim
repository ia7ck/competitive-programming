import strutils, sequtils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let n, m = read().parseInt
  var g = newSeqWith(n, newSeq[int]())
  for i in 0..<m:
    let a, b = read().parseInt
    g[a - 1].add(b - 1)
    g[b - 1].add(a - 1)

  var color = newSeqWith(n, -1)
  proc dfs(i, c: int) =
    color[i] = c
    for j in g[i]:
      if color[j] == -1:
        dfs(j, c xor 1)
  dfs(0, 0)
  var ok = true
  for i in 0..<n:
    for j in g[i]:
      if color[i] == color[j]:
        ok = false
  let nn = n.int64
  if ok:
    let
      w = color.filterIt(it == 0).len.int64
      b = color.filterIt(it == 1).len.int64
    echo (w * b - m)
  else:
    echo (n * (n - 1) div 2 - m)
main()
