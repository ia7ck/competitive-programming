import strutils, sequtils

proc main() =
  let n = stdin.readLine.strip.parseInt

  for i in (n - 7)..n:
    echo i
main()
