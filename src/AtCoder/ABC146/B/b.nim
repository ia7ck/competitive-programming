import strutils, sequtils

proc main() =
  let
    n = stdin.readLine.strip.parseInt
    s = stdin.readLine.strip

  var t = ""
  for c in s:
    let d = (c.ord - 'A'.ord + n) mod 26
    t = t & chr(d + 'A'.ord)
  echo t
main()
