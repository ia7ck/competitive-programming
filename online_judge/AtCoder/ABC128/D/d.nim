import strutils, sequtils, algorithm, math

proc main() =
  var n, k: int
  (n, k) = stdin.readLine.strip.split.map(parseInt)
  let v = stdin.readLine.strip.split.map(parseInt)

  var ans = 0
  for a in 0..min(n, k):
    for b in 0..min(n, k):
      if a + b > min(n, k): break
      var jewels = newSeq[int]()
      jewels.add(v[0..<a])
      jewels.add(v[(^1 - b + 1)..^1])
      jewels.sort(system.cmp)
      let m = k - (a + b)
      var s = jewels.sum
      for i in 0..<min(jewels.len, m):
        if jewels[i] >= 0:
          break
        s -= jewels[i]
      ans = max(ans, s)
  echo ans

main()
