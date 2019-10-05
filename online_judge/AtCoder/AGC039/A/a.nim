import strutils, sequtils, algorithm

proc solve(t: string, k, n: int): int64 =
  var s = t
  var cnt: int64 = 0
  for i in 1..<n:
    if s[i - 1] == s[i]:
      s[i] = '?'
      cnt += 1
  cnt *= k
  if s[0] == s[^1]:
    cnt += (k - 1)
  return cnt

proc main() =
  let
    s = stdin.readLine.strip
    k = stdin.readLine.strip.parseInt
    n = s.len

  if n == 1:
    echo (k div 2)
    return
  if s.allCharsInset({s[0]}):
    echo (k.int64 * n div 2)
    return
  let ans = min(solve(s, k, n), solve(s.reversed.mapIt($it).join(""), k, n))
  echo ans
main()
