import strutils, sequtils

proc main() =
  let
    nk = stdin.readLine.strip.split.map(parseInt)
    (n, k) = (nk[0], nk[1])
  echo n * k + 1 - k * (k - 1) div 2
main()
