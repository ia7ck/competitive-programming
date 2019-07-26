import strutils, sequtils

proc main() =
  let s = stdin.readLine.strip

  var
    a: int64 = 0
    tot: int64 = 0
    last = newSeqWith(26, -1)
  for i, c in s:
    let d = c.ord - 'a'.ord
    a = a + (i - last[d])
    tot += a
    last[d] = i

  let
    n: int64 = s.len
    res = tot.float64 / (n * (n + 1) div 2).float64
  echo res.formatFloat
main()
