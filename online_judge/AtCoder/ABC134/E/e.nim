import strutils, sequtils, algorithm, tables

proc main() =
  let n = stdin.readLine.strip.parseInt
  var a = (0..<n).mapIt(stdin.readLine.strip.parseInt)

  var b = a
  b.sort(system.cmp)
  var t = initTable[int, int]()
  for it in b:
    if not t.hasKey(it):
      t[it] = t.len
  for i in 0..<n:
    a[i] = t[a[i]]
  a.reverse
  var dp = newSeq[int](n + 2)
  fill(dp, 1000000000)
  dp[0] = 0
  for it in a:
    let ub = dp.lowerBound(it + 1)
    dp[ub] = it
  var ans = n
  for i in countdown(n, 1):
    if dp[i] < 1000000000:
      ans = i
      break
  echo ans
main()
