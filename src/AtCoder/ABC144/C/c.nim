import strutils, sequtils, math

proc main() =
  let n = stdin.readLine.strip.parseBiggestInt

  var ans = n - 1
  for i in 1..n:
    if i * i > n: break
    if n mod i == 0:
      # (i, n div i)
      ans = min(ans, (i - 1) + (n div i - 1))
  echo ans
main()
