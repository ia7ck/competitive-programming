import strutils, sequtils

proc main() =
  let s = stdin.readLine.strip

  var ans = "~"
  for ch in 'a'..'z':
    if s.find(ch) < 0:
      if (s & ch) > s:
        ans = min(ans, s & ch)
  var seen = newSeq[bool](26)
  for i, ch in s:
    for d in ch..'z':
      if d > ch and (not seen[d.ord - 'a'.ord]):
        let t = s[0..<i] & d
        if t > s:
          ans = min(ans, t)
    seen[ch.ord - 'a'.ord] = true
  if ans != "~":
    echo ans
  else:
    echo (-1)
main()
