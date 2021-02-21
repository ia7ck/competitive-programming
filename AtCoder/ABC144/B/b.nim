import strutils, sequtils

proc main() =
  let n = stdin.readLine.strip.parseInt

  var ok = false
  for a in 1..9:
    for b in 1..9:
      if a * b == n:
        ok = true
  if ok:
    echo "Yes"
  else:
    echo "No"
main()
