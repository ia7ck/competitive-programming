import strutils, sequtils
proc main() =
  let
    n = stdin.readLine.parseInt
    s = stdin.readLine

  var
    len = 0
    mx_len = 0
    ans = 0.0
  for c in s:
    if c=='>': # 氷
      ans+=(1.0/(len+2).float64)
      len+=1
    else: # 雪
      ans+=1.0
      len = 0
    mx_len = max(mx_len, len)
  ans-=1.0
  ans+=(1.0/(mx_len+2).float64)
  echo ans
main()
