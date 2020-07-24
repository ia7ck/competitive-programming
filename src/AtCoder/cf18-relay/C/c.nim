import strutils, sequtils, algorithm

proc main() =
  let
    nh = stdin.readLine.split.map(parseInt)
    (n, h) = (nh[0], nh[1])
    hi = (0..<n).mapIt(stdin.readLine.parseInt)

  var
    ids = (0..<n).mapIt(it)
    tot = 0
  while true:
    var
      used = newSeqWith(n, false)
      can = true
    for j in ids:
      var h_sum = 0
      for i in 0..j:
        if not used[i]:
          h_sum += hi[i]
      if h_sum > h:
        can = false
      used[j] = true
    if can:
      tot += 1
    if not ids.nextPermutation():
      break
  echo tot

main()
