import strutils, sequtils
proc main() =
  let
    n = stdin.readLine.strip.parseInt
    a = stdin.readLine.strip.split.map(parseInt)

  const mo: int64 = 1000000000 + 7
  var
    freq = newSeq[int64](n + 1)
    ans: int64 = 1
  for it in a:
    let f = freq[it]
    if it == 0:
      ans = ans * (3 - f) mod mo
    else:
      ans = ans * (freq[it - 1] - f) mod mo
    freq[it] += 1
  echo ans

main()
