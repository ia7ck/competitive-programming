import strutils, sequtils

proc main() =
  let s = stdin.readLine.strip
  if s[0] == 'S':
    echo "Cloudy"
  elif s[0] == 'C':
    echo "Rainy"
  else:
    echo "Sunny"

main()
