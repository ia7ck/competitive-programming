import strutils, sequtils

proc main() =
  let s = stdin.readLine.strip

  let n = s.len
  var
    a = 0
    ans: int64 = 0
    i = 0
  while i + 1 < n:
    if s[i] == 'A':
      a += 1
      i += 1
    elif s[i] == 'B' and s[i + 1] == 'C':
      ans += a
      i += 2
    else:
      a = 0
      i += 1
  echo ans
main()
