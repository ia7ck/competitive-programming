import strutils, sequtils

proc main() =
  let s = stdin.readLine
  var ans = 1_000_000_000
  for i in 0..<len(s):
    let t = s[i..<(i+3)]
    ans = min(ans, abs(753-parseInt(t)))
  echo ans

main()
