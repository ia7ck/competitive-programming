import strutils, sequtils
proc main() =
  let s = stdin.readLine.strip
  var
    b = 0'i64
    ans = 0'i64
  for c in s:
    if c == 'B':
      b+=1
    else:
      ans+=b
  echo ans
main()
