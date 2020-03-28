import strutils, sequtils, algorithm

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    n, m = read().parseInt
    s = read()

  var
    left = newSeq[int](n + 1)
    last = -1
  for i in 0..n:
    if s[i] == '0':
      last = i
    left[i] = last
  var d = newSeq[int](n + 1)
  const inf = 1234567890
  fill(d, inf)
  d[0] = 0
  for i in 0..n:
    let to = left[min(n, i + m)]
    d[to] = min(d[to], d[i] + 1)
  if d[n] == inf:
    echo "-1"
    return
  # echo left
  # echo d
  last = d[n]
  for i in countdown(n, 0):
    if d[i] < inf:
      last = d[i]
    if s[i] == '0':
      d[i] = last
  # echo d
  var pos = newSeq[int]()
  pos.add(n)
  while pos[^1] > 0:
    var nxt = -1
    for i in countdown(pos[^1] - 1, 0):
      if pos[^1] - i > m:
        break
      if d[i] + 1 == d[pos[^1]]:
        nxt = i
    doAssert(nxt >= 0)
    pos.add(nxt)
  # echo pos
  pos.reverse
  var ans = newSeq[int]()
  for i in 1..<pos.len:
    ans.add(pos[i] - pos[i - 1])
  echo ans.mapIt($it).join(" ")
main()
