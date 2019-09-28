import strutils, sequtils

proc main() =
  let n = stdin.readLine.strip.parseInt
  if n mod 2 == 0:
    echo "0.5"
  else:
    echo ((((n div 2) + 1) / n).formatFloat)
main()
