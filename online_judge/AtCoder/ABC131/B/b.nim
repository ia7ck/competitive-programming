import strutils, sequtils, math

proc main() =
  var n, k: int
  (n, k) = stdin.readLine.strip.split.map(parseInt)
  var a = newSeq[int](n)
  for i in 0..<n:
    a[i] = i + k
  let s = a.sum
  var
    mn = 1000000000
    ans = s - a[0]
  for i in 0..<n:
    let tmp = abs(s - (s - a[i])) # ???
    if tmp < mn:
      mn = tmp
      ans = s - a[i]
  echo ans

main()
