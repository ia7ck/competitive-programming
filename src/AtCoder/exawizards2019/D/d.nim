import strutils, sequtils, algorithm

const mo = 1000000000 + 7

proc add(a: var int64, b: int64) =
  a = (a + b) mod mo

var memo: seq[seq[int]]
proc f(i, x: int, a: seq[int]): int =
  let n = a.len
  if i == n:
    return x
  else:
    if memo[i][x] < 0:
      memo[i][x]
        = (f(i + 1, x mod a[i], a) + f(i + 1, x, a) * (n - i - 1)) mod mo
    return memo[i][x]

proc main() =
  let
    nx = stdin.readLine.strip.split.map(parseInt)
    (n, x) = (nx[0], nx[1])
  var a = stdin.readLine.strip.split.map(parseInt)

  a.sort(system.cmp)
  a.reverse
  memo = newSeqWith(n + 1, newSeqWith(x + 1, -1))
  echo f(0, x, a)

main()
