import strutils, sequtils

proc main() =
  let n = stdin.readLine.strip.parseInt
  var a = stdin.readLine.strip.split.map(parseInt)

  a = @[-1] & a
  var b = newSeq[int](n + 1)
  for i in countdown(n, 1):
    var
      s = 0
      j = i * 2
    while j <= n:
      s += b[j]
      j += i
    if (s mod 2) == a[i]:
      b[i] = 0
    else:
      b[i] = 1

  var ans = newSeq[int]()
  for i in 1..n:
    if b[i] == 1:
      ans.add(i)
  echo ans.len
  if ans.len > 0:
    echo ans.mapIt($it).join(" ")
main()
