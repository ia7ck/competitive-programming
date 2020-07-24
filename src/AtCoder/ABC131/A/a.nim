import strutils, sequtils

proc main() =
  let s = stdin.readLine.strip
  for i in 0..<3:
    if s[i] == s[i + 1]:
      echo "Bad"
      return
  echo "Good"
main()
