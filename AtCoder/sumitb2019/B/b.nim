import strutils, sequtils

proc main() =
  let n = stdin.readLine.strip.parseInt

  for x in 1..n:
    let y = x * 108 div 100
    if y == n:
      echo x
      return
  echo ":("
main()
