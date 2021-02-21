import strutils, sequtils
proc main() =
  let n = stdin.readLine.strip.parseInt
  let s = stdin.readLine.strip

  var c = 0
  for i in 2..<n:
    if s[(i - 2)..i] == "ABC":
      c += 1
  echo c
main()
