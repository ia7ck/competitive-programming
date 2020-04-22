import strutils, sequtils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let n = read().parseInt
  type E = tuple[to, cost: int]
  var g = newSeqWith(n, newSeq[E]())
  for i in 1..<n:
    let u, v, w = read().parseInt
    g[u - 1].add((v - 1, w mod 2))
    g[v - 1].add((u - 1, w mod 2))
  var ans = newSeq[int](n)
  proc dfs(i, p, c: int) =
    ans[i] = c
    for e in g[i]:
      if e.to != p:
        dfs(e.to, i, (c + e.cost) mod 2)
  dfs(0, -1, 0)
  echo ans.mapIt($it).join("\n")
main()
