import strutils, sequtils, math

proc main() =
  let
    n = stdin.readLine.strip.parseInt
    s = stdin.readLine.strip
  var
    white = newSeq[int](n + 1)
    black = newSeq[int](n + 1)
  for i in 0..<n:
    if s[i] == '.':
      white[i + 1] += 1
    else:
      black[i + 1] += 1
    white[i + 1] += white[i]
    black[i + 1] += black[i]
  var ans = n
  for i in 0..n:
    # 0..<i white
    # i..<n black
    ans = min(ans, black[i] + (white[n] - white[i]))
  echo ans
main()
