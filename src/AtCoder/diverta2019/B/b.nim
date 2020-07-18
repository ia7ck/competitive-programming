import strutils, sequtils

proc main() =
  var r, g, b, n: int
  (r, g, b, n) = stdin.readLine.strip.split.map(parseInt)

  var ans = 0
  for i in 0..3000:
    for j in 0..3000:
      let k = n - i * r - j * g
      if k >= 0 and k mod b == 0:
        ans += 1
  echo ans
main()
