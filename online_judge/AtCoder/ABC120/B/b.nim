import strutils, sequtils

proc main() =
  let
    abk = stdin.readLine.strip.split.map(parseInt)
    (a, b, k) = (abk[0], abk[1], abk[2])

  var m = 0
  for i in countdown(100, 1):
    if a mod i == 0 and b mod i == 0:
      m += 1
      if m == k:
        echo i
        return
  doAssert(false)
main()
