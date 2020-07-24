import strutils, sequtils, tables, algorithm

proc main() =
  let n = stdin.readLine.strip.parseInt
  var freq = initTable[string, int]()
  for i in 0..<n:
    let s = stdin.readLine.strip
    if freq.hasKey(s):
      freq[s] += 1
    else:
      freq[s] = 1
  var mx = 0
  for v in freq.values:
    mx = max(mx, v)
  var ans = newSeq[string]()
  for k in freq.keys:
    if freq[k] == mx:
      ans.add(k)
  ans.sort(cmp)
  echo ans.join("\n")
main()
