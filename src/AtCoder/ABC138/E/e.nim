import strutils, sequtils

proc possible(s, t: string): bool =
  var seen = newSeq[bool](26)
  for ch in s:
    let d = ch.ord - 'a'.ord
    seen[d] = true
  for ch in t:
    let d = ch.ord - 'a'.ord
    if not seen[d]:
      return false
  return true

proc main() =
  let
    s = stdin.readLine.strip
    t = stdin.readLine.strip

  if not possible(s, t):
    echo "-1"
    return

  let
    n = s.len
    ss = s & s
  var
    nxt = newSeqWith(26, newSeqWith(n, -1))
    last = newSeqWith(26, -1)
  for i in countdown((n * 2 - 1), 0):
    let d = ss[i].ord - 'a'.ord
    last[d] = i mod n
    if i < n: # ???
      for j in 0..<26:
        nxt[j][i] = last[j]
  #
  # echo nxt['t'.ord - 'a'.ord] # => @[3, 3, 3, 3, 6, 6, 6]
  # echo nxt['o'.ord - 'a'.ord] # => @[1, 1, 1, 1, 1, 1, 1]
#
  var
    i = 0
    ans: int64 = 0
  for ch in t:
    let d = ch.ord - 'a'.ord
    if i <= nxt[d][i]:
      ans += nxt[d][i] - i + 1
    else:
      ans += ((n - 1) - i + 1) + (nxt[d][i] + 1)
    i = (nxt[d][i] + 1) mod n
  echo ans

main()
