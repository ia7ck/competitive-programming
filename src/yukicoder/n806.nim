import strutils, sequtils

proc main() =
  let
    n = stdin.readLine.strip.parseInt
    abi = (0..<(n - 1)).mapIt(stdin.readLine.strip.split.map(parseInt))
  var deg = newSeq[int](n + 1)
  for ab in abi:
    deg[ab[0]] += 1
    deg[ab[1]] += 1
  var ans = 0
  for i in 1..n:
    ans += max(0, deg[i] - 2)
  echo ans
main()
