import strutils, sequtils
proc main() =
  let n = stdin.readLine.strip.parseInt
  var c = (0..<n).mapIt(stdin.readLine.strip)
  var ans = 0
  for i in 0..<n:
    let
      le = c[i].find('.')
      ri = c[i].rfind('.')
    if le == -1: continue
    for j in 0..<n:
      if j <= ri:
        c[i][j] = 'o'
      if j >= ri and i + 1 < n:
        c[i + 1][j] = 'o'
    ans += 1
  echo ans
main()
