import strutils, sequtils
proc main() =
  var t1, t2: int64
  (t1, t2) = stdin.readLine.strip.split.map(parseBiggestInt)
  var a1, a2: int64
  (a1, a2) = stdin.readLine.strip.split.map(parseBiggestInt)
  var b1, b2: int64
  (b1, b2) = stdin.readLine.strip.split.map(parseBiggestInt)

  if a1 < b1:
    swap(a1, b1)
    swap(a2, b2)
  let p = (a1 - b1) * t1 - (b2 - a2) * t2
  if p == 0:
    echo "infinity"
    return
  if p > 0:
    echo 0
    return
  let
    q = (a1 - b1) * t1 div (p * (-1))
    r = (a1 - b1) * t1 mod (p * (-1))
    ans = q * 2 + (if r == 0: 0 else: 1)
  echo ans
main()
