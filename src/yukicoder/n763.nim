import strutils, sequtils

var
  g: seq[seq[int]]
  ans = 0

proc f(i, p: int): bool = # 頂点iを残すか
  result = true
  for j in g[i]:
    if j!=p:
      if f(j, i):
        result = false # 子が1つでも残っていれば頂点iを削除
  if result: ans+=1

proc main() =
  let n = stdin.readLine.parseInt
  g = newSeqWith(n, newSeq[int]())
  for _ in 1..<n:
    let ab = stdin.readLine.split.map(parseInt)
    g[ab[0]-1].add(ab[1]-1)
    g[ab[1]-1].add(ab[0]-1)
  let _ = f(0, -1)
  echo ans
main()
