import strutils, sequtils, tables, algorithm

proc main() =
  let n = stdin.readLine.strip.parseInt
  var freq = initTable[string, int64]()
  for i in 0..<n:
    var s = stdin.readLine.strip
    s.sort(cmp)
    if freq.hasKey(s):
      freq[s] += 1
    else:
      freq[s] = 1
  var ans: int64 = 0
  for v in freq.values:
    ans += v * (v - 1) div 2
  echo ans
main()
