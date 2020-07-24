import strutils, sequtils, future

type T = object
  t: char
  d: char

proc enoughRight(initialPos: int, s: string, tdi: seq[T]): bool =
  var k = initialPos
  for td in tdi:
    if s[k] == td.t:
      if td.d == 'L':
        k -= 1
      else:
        k += 1
      if k < 0:
        return false
  return true

proc enoughLeft(initialPos: int, s: string, tdi: seq[T]): bool =
  var k = initialPos
  for td in tdi:
    if s[k] == td.t:
      if td.d == 'L':
        k -= 1
      else:
        k += 1
      if k >= s.len:
        return false
  return true

proc main() =
  let
    nq = stdin.readLine.strip.split.map(parseInt)
    (n, q) = (nq[0], nq[1])
    s = stdin.readLine.strip
  var tdi = newSeq[T](q)
  for i in 0..<q:
    let td = stdin.readLine.strip.split.map(it => it[0])
    tdi[i] = T(t: td[0], d: td[1])

  var
    ok = n                    # 左側はここまで落ちない
    ng = -1                   # ここまで落ちる
  while ok - ng > 1:
    let md = (ok + ng) div 2
    if enoughRight(md, s, tdi):
      ok = md
    else:
      ng = md
  if ok == n: # ぜんぶ落ちた
    echo 0
    return

  let left = ok

  ok = -1
  ng = n
  while ng - ok > 1:
    let md = (ng + ok) div 2
    if enoughLeft(md, s, tdi):
      ok = md
    else:
      ng = md
  if ok == -1: # ぜんぶ落ちた
    echo 0
    return

  echo max(0, ok - left + 1)

main()
