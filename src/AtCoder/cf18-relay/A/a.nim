import strutils, sequtils

let
  n = stdin.readLine.parseInt
  s = (0..<n).mapIt(stdin.readLine)
  days = @["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]

for d in s:
  var j = find(days, d)
  j = (j+1) mod len(days)
  echo days[j]
