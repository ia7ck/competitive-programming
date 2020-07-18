import strutils, sequtils

proc main() =
  var n, m: int
  (n, m) = stdin.readLine.strip.split.map(parseInt)
  var
    s = newSeq[int](m)
    c = newSeq[int](m)
  for i in 0..<m:
    (s[i], c[i]) = stdin.readLine.strip.split.map(parseInt)

  for x in 0..999:
    let y = $x
    if y.len == n:
      var ok = true
      for j in 0..<m:
        if (y[s[j] - 1].ord - '0'.ord) != c[j]:
          ok = false
      if ok:
        echo x
        return
  echo -1
main()
