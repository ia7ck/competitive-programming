import strutils, sequtils, math

proc main() =
  let
    nk = stdin.readLine.strip.split.map(parseBiggestInt)
    (n, k) = (nk[0].int, nk[1])
    a = stdin.readLine.strip.split.map(parseBiggestInt)
  var ans: int64 = 0
  for it in a:
    ans += (it xor k)
  ans = max(ans, a.sum)
  for i in 0..<50:
    if (k and (1 shl i)) > 0:
      var s: int64 = 0
      for j in 0..<50:
        var popcnt = 0
        for elm in a:
          if (elm and (1 shl j)) > 0:
            popcnt += 1
        if (j > i) and ((k and (1 shl j)) > 0):
          popcnt = n - popcnt
        elif j < i:
          popcnt = max(popcnt, n - popcnt)
        s += popcnt * (1 shl j)
      ans = max(ans, s)
  echo ans
main()
