
import strutils, sequtils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let
    s = read()
    t = read()
    n = s.len
    m = s.len
  var freq = newSeq[int](26)
  for c in s:
    freq[c.ord - 'a'.ord] += 1
  for c in t:
    if freq[c.ord - 'a'.ord] == 0:
      echo -1
      return
  const inf = int.high
  var p = newSeqWith(26, newSeqWith(n, inf))
  for c in 'a'..'z':
    let d = c.ord - 'a'.ord
    for i in countdown(n - 1, 0):
      if i + 1 < n:
        p[d][i] = min(p[d][i], p[d][i + 1])
      if s[i] == c:
        p[d][i] = i
  var
    j = 0
    ans: int64 = 0
  for c in t:
    let d = c.ord - 'a'.ord
    if p[d][j] < inf:
      ans += p[d][j] - j + 1
      j = (p[d][j] + 1) mod n
    else:
      ans += n - j + 1
      # j = 0
      ans += p[d][0]
      j = p[d][0] + 1
    # echo j, " ", ans
  echo ans
main()
