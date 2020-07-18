import strutils, sequtils

proc can(a: seq[int], k, maxLen: int): bool =
  var
    pre = -1
    len = 0
    m = 0
  for j, it in a:
    if pre == it:
      if len < maxLen:
        len += 1
        pre = it
      elif m < k:
        m += 1
        len = 1
        pre = j * (-1)
      else:
        return false
    else:
      len = 1
      pre = it
  return true

proc main() =
  var n, k: int
  (n, k) = stdin.readLine.strip.split.map(parseInt)
  let a = stdin.readLine.strip.split.map(parseInt)

  var
    ok = n
    ng = 0
  while ok - ng > 1:
    let m = (ok + ng) div 2
    if can(a, k, m):
      ok = m
    else:
      ng = m
  echo ok

main()
