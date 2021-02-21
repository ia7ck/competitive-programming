import strutils, sequtils

proc main() =
  let
    k = stdin.readLine.strip.parseInt
    a = stdin.readLine.strip.split.map(parseBiggestInt)
  var
    mn: int64 = 2
    mx: int64 = 2
  for i in countdown(k-1, 0):
    mn = max(mn, ((mn + a[i] - 1) div a[i]) * a[i])
    mx = max(mx, ((mx div a[i]) + 1) * a[i] - 1)
  if mn > mx:
    echo -1
    return
  let
    ans_mn = mn
    ans_mx = mx
  for it in a:
    mn = mn - mn mod it
    mx = mx - mx mod it
  if mn != 2 or mx != 2:
    echo -1
  else:
    echo ans_mn, " ", ans_mx

main()
