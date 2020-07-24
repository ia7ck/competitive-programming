import strutils, sequtils

proc main() =
  let
    qhsd = stdin.readLine.strip.split.map(parseBiggestInt)
    (q, h, s, d) = (qhsd[0], qhsd[1], qhsd[2], qhsd[3])
    n = stdin.readLine.strip.parseBiggestInt

  let ss = min(q*4, h*2, s)
  var ans: int64
  if ss * 2 <= d:
    ans = ss * n
  else:
    ans = d * (n div 2)
    if n mod 2 == 1:
      ans += ss
  echo ans
main()
