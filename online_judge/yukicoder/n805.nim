import strutils, sequtils

proc main() =
  let
    n = stdin.readLine.strip.parseInt
    s = stdin.readLine.strip

  var ans: int64 = 0
  for i in 0..<n:
    for k in (i + 2)..<n:
      if s[i] == 'U' and s[k] == 'G':
        if (i + k) mod 2 == 0 and s[(i + k) div 2] == 'M':
          ans += 1
  echo ans
main()
