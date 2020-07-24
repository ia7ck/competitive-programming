import strutils, sequtils, tables
proc main() =
  let
    np = stdin.readLine.split.map(parseBiggestInt)
  var
    (n, p) = (np[0], np[1])
    tb = newTable[int64, int]()
    j = 2'i64
  while j*j<=p:
    while p mod j == 0:
      if tb.hasKey(j):
        tb[j]+=1
      else:
        tb[j] = 1
      p = p div j
    j+=1
  if p>1:
    tb[p] = 1
  var ans = 1'i64
  for k, v in tb:
    for _ in 0..<(v div n).int:
      ans = ans*k
  echo ans


main()
