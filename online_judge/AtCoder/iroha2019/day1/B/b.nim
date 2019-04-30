import strutils, sequtils

proc main() =
  var
    s = stdin.readLine.strip
    k = stdin.readLine.strip.parseInt

  let n = s.len
  var t = newSeq[char](n)
  for i in 0..<n:
    t[(i + (n - k mod n)) mod n] = s[i]
  echo t.mapIt($it).join

main()
