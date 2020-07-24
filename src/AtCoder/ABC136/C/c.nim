import strutils, sequtils

proc main() =
  let n = stdin.readLine.strip.parseInt
  var a = stdin.readLine.strip.split.map(parseInt)

  a[0] -= 1
  for i in 1..<n:
    if a[i - 1] <= a[i] - 1:
      a[i] -= 1
  var ok = true
  for i in 1..<n:
    if a[i - 1] > a[i]:
      ok = false
  if ok:
    echo "Yes"
  else:
    echo "No"
main()
