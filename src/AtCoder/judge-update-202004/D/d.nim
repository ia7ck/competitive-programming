import strutils, math, sequtils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let
    n, q = read().parseInt
    a = newSeqWith(n, read().parseInt)
  var cul = newSeq[int](n)
  cul[0] = a[0]
  for i in 1..<n:
    cul[i] = gcd(cul[i - 1], a[i])
  for qq in 0..<q:
    let
      x = read().parseInt
      y = gcd(x, cul[n - 1])
    if y > 1:
      echo y
      continue
    var
      ok = -1
      ng = n - 1
    while ng - ok > 1:
      let md = (ng + ok) div 2
      if gcd(cul[md], x) == 1:
        ng = md
      else:
        ok = md
    echo ng + 1
main()
