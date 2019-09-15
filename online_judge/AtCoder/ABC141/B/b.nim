import strutils, sequtils

proc main() =
  let s = stdin.readLine.strip

  var
    od = true
    ev = true
  for i in 0..<s.len:
    if (i mod 2) == 0:
      if s[i] == 'L':
        od = false
    else:
      if s[i] == 'R':
        ev = false
  if od and ev:
    echo "Yes"
  else:
    echo "No"

main()
