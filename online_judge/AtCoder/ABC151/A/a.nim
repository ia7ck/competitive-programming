import strutils, sequtils
proc main() =
  let c = stdin.readLine.strip
  echo ((c[0].ord + 1).chr)
main()
