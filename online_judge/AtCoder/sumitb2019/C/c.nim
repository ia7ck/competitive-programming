import strutils, sequtils

proc main() =
  let x = stdin.readLine.strip.parseInt

  var items = @[100, 101, 102, 103, 104, 105]
  var dp = newSeq[bool](x + 1)
  dp[0] = true
  for y in 0..<x:
    if not dp[y]: continue
    for it in items:
      var z = y + it
      while z <= x:
        dp[z] = true
        z += it
  if dp[x]:
    echo 1
  else:
    echo 0
main()
