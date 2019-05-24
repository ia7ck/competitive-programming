import strutils, sequtils

proc rec(f: seq[int]): bool =
  if f.allIt(it == 0):
    return true
  for i in 0..9:
    if f[i] > 0:
      if f[i] >= 3:
        if rec(f[0..<i] & (f[i] - 3) & f[(i + 1)..^1]):
          return true
      if i + 2 > 9:
        return false
      if f[i + 1] < f[i] or f[i + 2] < f[i]:
        return false
      let tmp = f[i]
      if rec(f[0..<i] & f[i..(i + 2)].mapIt(it - f[i]) & f[(i + 3)..^1]):
        return true
  return false

proc check(a: seq[int]): bool =
  var freq = newSeq[int](10)
  for it in a:
    freq[it - 1] += 1
  if freq.filterIt(it == 2).len == 7:
    return true
  for i in 0..9:
    if freq[i] >= 2:
      if rec(freq[0..<i] & (freq[i] - 2) & freq[(i + 1)..^1]):
        return true
  return false


proc main() =
  let s = stdin.readLine.strip
  var a = newSeq[int]()
  for ch in s:
    a.add(ch.ord - '0'.ord)
  var ans = newSeq[int]()
  for i in 1..9:
    if a.filterIt(it == i).len < 4:
      if check(a & i):
        ans.add(i)
  echo ans.mapIt($it).join("\n")
main()
