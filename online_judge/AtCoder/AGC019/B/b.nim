import strutils, sequtils

proc main() =
  let s = stdin.readLine.strip

  var freq = newSeq[int64](26)
  for ch in s:
    freq[ch.ord - 'a'.ord] += 1
  var ans: int64 = 0
  for ch in s:
    for d in 0..<26:
      if ch.ord - 'a'.ord != d:
        ans += freq[d]
    freq[ch.ord - 'a'.ord] -= 1
  echo ans + 1
main()
