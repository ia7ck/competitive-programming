import strutils, sequtils, algorithm

proc main() =
  let n = stdin.readLine.strip.parseInt
  var a = stdin.readLine.strip.split.map(parseInt)

  var freq = newSeq[int](100005)
  for it in a:
    freq[it] += 1
  a.sort(system.cmp)
  a.reverse
  var
    arc = 0
    ans = 0
  for k in countdown(100001, 1):
    arc += freq[k]
    if arc == n div 2:
      ans += 1
  echo ans
main()
