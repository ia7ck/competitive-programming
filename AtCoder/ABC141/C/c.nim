import strutils, sequtils

proc main() =
  var n, k, q: int
  (n, k, q) = stdin.readLine.strip.split.map(parseInt)
  let a = (0..<q).mapIt(stdin.readLine.strip.parseInt)

  var score = newSeqWith(n, k - q)
  for it in a:
    score[it - 1] += 1
  echo score.mapIt(if it > 0: "Yes" else: "No").join("\n")

main()
