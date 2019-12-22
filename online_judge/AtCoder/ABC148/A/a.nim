import strutils, sequtils
proc main() =
  var
    a = stdin.readLine.strip.parseInt
    b = stdin.readLine.strip.parseInt
  if a > b:
    swap(a, b)
  if a == 1:
    if b == 2:
      echo 3
    else:
      echo 2
  else:
    echo 1
main()
