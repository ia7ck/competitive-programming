import strutils, sequtils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let n = read().parseInt
  var g = newSeqWith(n, newSeq[int]())
  for i in 1..<n:
    let a, b = read().parseInt
    g[a - 1].add(b - 1)
    g[b - 1].add(a - 1)
  proc dfs(i, p: int, d: var seq[int])=
    for j in g[i]:
      if j != p:
        d[j] = d[i] + 1
        dfs(j, i, d)
  var
    dB = newSeq[int](n)
    dW = newSeq[int](n)
  dfs(0, -1, dB)
  dfs(n - 1, -1, dW)
  var b = 0
  for i in 0..<n:
    if dB[i] <= dW[i]:
      b += 1
  if b >= n div 2 + 1:
    echo "Fennec"
  else:
    echo "Snuke"
main()
