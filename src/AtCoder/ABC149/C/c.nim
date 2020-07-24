import strutils, sequtils
proc main() =
  let n = stdin.readLine.strip.parseBiggestInt
  for i in n..<10000000:
    var
      ok = true
      j = 2
    while j * j <= i:
      if i mod j == 0:
        ok = false
        break
      j += 1
    if ok:
      echo i
      return
  doAssert(false)
main()
