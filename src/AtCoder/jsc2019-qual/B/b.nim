import strutils, sequtils

proc main() =
  var n, k: int
  (n, k) = stdin.readLine.strip.split.map(parseInt)
  let a = stdin.readLine.strip.split.map(parseInt)

  const mo: int64 = 1000000000 + 7
  let
    c = k * (k + 1) div 2 mod mo
    d = (k - 1) * k div 2 mod mo
  var ans: int64 = 0
  for i in 0..<n:
    var
      left: int64 = 0
      right: int64 = 0
    for j in 0..<i:
      if a[j] > a[i]:
        left += 1
    for j in (i + 1)..<n:
      if a[i] < a[j]:
        right += 1
    ans += (left * c mod mo)
    ans = ans mod mo
    ans += (right * d mod mo)
    ans = ans mod mo
  echo ans
main()
