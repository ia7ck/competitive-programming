import strutils, sequtils
proc main() =
  var n, m, d: float64
  (n, m, d) = stdin.readLine.strip.split.map(parseFloat)

  let a = if d == 0: n else: (n - d) * 2
  echo ((a / n / n * (m - 1)).formatFloat)
main()
