import strutils, sequtils

proc main() =
  let
    s = stdin.readLine.strip
    d = @["SUN", "MON", "TUE", "WED", "THU", "FRI", "SAT"]
    a = @[7, 6, 5, 4, 3, 2, 1]

  echo a[d.find(s)]
main()
