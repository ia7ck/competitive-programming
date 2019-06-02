import strutils, sequtils

proc can(a: string, s, t: int): bool =
  doAssert(s < t)
  let n = a.len
  var dp = newSeq[bool](n)
  dp[s] = true
  for i in s..<t:
    if not dp[i]: continue
    if i + 1 < n and a[i + 1] == '.':
      dp[i + 1] = dp[i + 1] or dp[i]
    if i + 2 < n and a[i + 2] == '.':
      dp[i + 2] = dp[i + 2] or dp[i]
  return dp[t]

proc main() =
  var n, a, b, c, d: int
  (n, a, b, c, d) = stdin.readLine.strip.split.map(parseInt)
  a -= 1
  b -= 1
  c -= 1
  d -= 1
  let s = stdin.readLine.strip

  if (not can(s, a, c)) or (not can(s, b, d)):
    echo "No"
    return
  var res: bool = false
  if c < d:
    res = true
  else:
    # a<b<d<c
    for i in a..d:
      if b <= i + 1 and i + 1 <= d and i + 2 < n:
        if s[i] == '.' and s[i + 1] == '.' and s[i + 2] == '.':
          res = true
          break

  if res:
    echo "Yes"
  else:
    echo "No"
main()
