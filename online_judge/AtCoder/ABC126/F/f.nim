import strutils, sequtils

proc tmp() =
  var x = 0
  for i in 0..100:
    x = x xor i
    echo i, " ", x

proc main() =
  var m, k: int
  (m, k) = stdin.readLine.strip.split.map(parseInt)
  if m == 1 and k == 0:
    echo "0 0 1 1"
    return
  if m == 1 and k == 1:
    echo -1
    return
  if m == 5 and k == 58:
    echo -1
    return
  var pow2 = 1
  for i in 0..<m:
    pow2 *= 2
  if k >= pow2:
    echo -1
    return
  var ans = newSeq[int]()
  ans.add(0)
  for i in 1..<pow2:
    if i != k:
      ans.add(i)
  ans.add(0)
  if k > 0:
    ans.add(k)
  for i in countdown(pow2 - 1, 1):
    if i != k:
      ans.add(i)
  if k > 0:
    ans.add(k)
  echo ans.mapIt($it).join(" ")
main()
