import strutils, sequtils

proc main() =
  let n = stdin.readLine.strip.parseInt

  echo ((n - 2) * 180)
main()
