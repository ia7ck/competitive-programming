import strutils, sequtils, math

proc main() =
  let n = stdin.readLine.strip.parseInt
  var strs = (0..<n).mapIt(stdin.readLine.strip)

  var
    a = 0
    b = 0
    ba = 0
    ans = 0
  for s in strs:
    var cnt = 0
    if s[0] == 'B' and s[^1] == 'A':
      ba += 1                 # B...A
    elif s[0] == 'B':
      b += 1                  # B...x
    elif s[^1] == 'A':
      a += 1                  # x...A
    var prev = '?'
    for ch in s:
      if prev == 'A' and ch == 'B':
        cnt += 1
      prev = ch
    ans += cnt
  if a == 0 and b == 0:
    echo ans + max(0, ba - 1)
    return
  while ba > 0:
    if a == 0 and b == 0:
      break
    if a > 0:
      ans += 1
      a -= 1
    if b > 0:
      ans += 1
      b -= 1
    ba -= 1
  if a == 0 and b == 0:
    ans += ba
  else:
    ans += min(a, b)

  echo ans
main()
