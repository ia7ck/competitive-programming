import strutils, sequtils

proc main() =
  var m, d: int
  (m, d) = stdin.readLine.strip.split.map(parseInt)

  var ans = 0
  for i in 1..m:
    for j in 1..d:
      let
        d1 = j mod 10
        d10 = ((j div 10) mod 10)
      if d1 >= 2 and d10 >= 2 and d1 * d10 == i:
        ans += 1
  echo ans
main()
