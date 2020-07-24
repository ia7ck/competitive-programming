import strutils, sequtils
proc can(s: string, w: int): bool =
  var t = 0
  result = true
  for i, c in s:
    if (t==0 and c!=s[0]) or (t==1 and c==s[0]):
      if max(i, s.len-i)<w:
        result = false
        break
      t = t xor 1

proc main() =
  let
    s = stdin.readLine
    n = s.len
  var
    (ok, ng) = (1, n+1)
  while ng-ok>1:
    let m = (ok+ng) div 2
    if can(s, m):
      ok = m
    else:
      ng = m
  echo ok
main()
