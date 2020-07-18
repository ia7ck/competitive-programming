import strutils, sequtils, algorithm

proc main() =
  let
    s = stdin.readLine.strip
    n = s.len

  var
    rcnt = 0
    lcnt = 0
    ans = newSeq[int](n)
  for i in 0..n:
    if (i == n) or (s[i] == 'R' and lcnt > 0):
      let
        ml = lcnt div 2
        mr = rcnt div 2
      ans[i - lcnt - 1] = ml + (rcnt - mr)
      ans[i - lcnt] = (lcnt - ml) + mr
      lcnt = 0
      rcnt = 1
    elif s[i] == 'R':
      rcnt += 1
    else:
      lcnt += 1
  echo ans.mapIt($it).join(" ")
main()
