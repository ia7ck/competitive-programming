import strutils, sequtils

proc main() =
  var n, m: int
  (n, m) = stdin.readLine.strip.split.map(parseInt)
  var lamps = newSeqWith(m, newSeq[int]())
  for i in 0..<m:
    let args = stdin.readLine.strip.split.map(parseInt)
    lamps[i] = args[1..^1].mapIt(it - 1)
  let p = stdin.readLine.strip.split.map(parseInt)
  var ans = 0
  for bits in 0..<(1 shl n):
    var all = true
    for i in 0..<m:
      var cnt = 0
      for j in lamps[i]:
        if (bits and (1 shl j)) > 0:
          cnt += 1
      if (cnt mod 2) != p[i]:
        all = false
        break
    if all:
      ans += 1
  echo ans
main()
