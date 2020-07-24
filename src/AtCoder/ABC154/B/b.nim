import strutils, sequtils
proc main() =
  let n = stdin.readLine.strip.len
  echo 'x'.repeat(n)
main()
