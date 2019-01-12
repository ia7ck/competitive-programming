import strutils, sequtils

proc ok(a, b, c: int): bool =
  result = (a!=b and b!=c and c!=a)
  result = result and ((a<b and b>c) or (a>b and b<c))

proc main() =
  let
    nc = stdin.readLine.split.map(parseInt)
    (n, c) = (nc[0], nc[1])
    ls = stdin.readLine.split.map(parseInt)
    ws = stdin.readLine.split.map(parseInt)
  var dp = newSeqWith(c+1, newSeqWith(51, newSeqWith(51, -1)))
  for i in 0..<n:
    for j in 0..<n:
      for k in 0..<n:
        if ok(ls[i], ls[j], ls[k]):
          let s = ws[i]+ws[j]+ws[k]
          if s<=c:
            dp[s][ls[j]][ls[k]] = max(
              dp[s][ls[j]][ls[k]], ls[i]+ls[j]+ls[k])
  for i in 0..c:
    for j in 0..<n:
      for k in 0..<n:
        if dp[i][ls[j]][ls[k]]>=0:
          for t in 0..<n:
            if ok(ls[j], ls[k], ls[t]):
              if i+ws[t]<=c:
                dp[i+ws[t]][ls[k]][ls[t]] = max(
                  dp[i+ws[t]][ls[k]][ls[t]], dp[i][ls[j]][ls[k]]+ls[t])
  var ans = 0
  for x in dp:
    for y in x:
      for z in y:
        ans = max(ans, z)
  echo ans
main()
