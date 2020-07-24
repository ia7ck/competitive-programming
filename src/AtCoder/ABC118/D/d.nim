import strutils, sequtils, algorithm
let match = @[0, 2, 5, 5, 4, 5, 6, 3, 7, 6]

proc mk(i: int, dp, a: seq[int]): string =
  if i == 0:
    result = ""
  else:
    for j in a:
      if i - match[j] >= 0 and dp[i] == dp[i-match[j]] + 1: # jを使える
        result = $j & mk(i-match[j], dp, a)
        break

proc main() =
  let
    nm = stdin.readLine.split.map(parseInt)
    (n, m) = (nm[0], nm[1])
  var a = stdin.readLine.split.map(parseInt)

  var dp = newSeqWith(n+1, -1) # i本使って作れる数の最大桁
  dp[0] = 0
  for i in 0..<n:
    for j in a:
      if i + match[j] <= n:
        dp[i+match[j]] = max(dp[i+match[j]], dp[i] + 1)
  a.sort(proc(x, y: int): int = cmp(y, x))
  echo mk(n, dp, a)
main()
