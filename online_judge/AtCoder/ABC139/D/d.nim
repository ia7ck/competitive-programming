import strutils, sequtils

proc main() =
  let n = stdin.readLine.strip.parseBiggestInt
  echo (n * (n - 1) div 2)
main()
