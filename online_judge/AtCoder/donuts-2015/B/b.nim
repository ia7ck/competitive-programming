import strutils, sequtils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let
    n, m = read().parseInt
    a = newSeqWith(n, read().parseInt)
  var
    b = newSeq[int](m)
    ids = newSeqWith(m, newSeq[int]())
  for i in 0..<m:
    b[i] = read().parseInt
    let c = read().parseInt
    ids[i] = newSeqWith(c, read().parseInt - 1)

  var ans = 0
  # [0, i)
  proc solve(i: int, ch: var seq[int]) =
    if i == n:
      if ch.len == 9:
        var s = 0
        for j in ch:
          s += a[j]
        for j in 0..<m:
          var cnt = 0
          for k in ids[j]:
            if ch.find(k) >= 0:
              cnt += 1
          if cnt >= 3:
            s += b[j]
        ans = max(ans, s)
    else:
      solve(i + 1, ch)
      if ch.len < 9:
        ch.add(i)
        solve(i + 1, ch)
        discard ch.pop

  var ch = newSeq[int]()
  solve(0, ch)
  echo ans
main()
