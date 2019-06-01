import strutils, sequtils

proc main() =
  let s = stdin.readLine.strip

  let w = s.filterIt(it == 'o').len
  if w + (15 - s.len) >= 8:
    echo "YES"
  else:
    echo "NO"
main()
