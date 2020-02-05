import strutils, sequtils, mersenne
var mt = newMersenneTwister(0)

proc solve(n: int, c: seq[string]) =
  var ans = 0
  for a in 0..<n:
    for b in 0..<n:
      var sym = true
      for i in 0..<n:
        for j in 0..<i:
          if c[(i + a) mod n][(j + b) mod n] != c[(j + a) mod n][(i + b) mod n]:
            sym = false
      if sym:
        ans += 1
  echo ans

proc sub() =
  const
    n = 5
    chs = @['a', 'a', 'a', 'a', 'a', 'a', 'b']
  var c = newSeq[string](n)
  for q in 0..10:
    for i in 0..<n:
      var s = ""
      for j in 0..<n:
        let k = mt.getNum mod chs.len
        s = s & chs[k]
      c[i] = s
    solve(n, c)

proc main() =
  # sub()
  let
    n = stdin.readLine.strip.parseInt
    c = (0..<n).mapIt(stdin.readLine.strip)
  var ans = 0
  for a in 0..<n:
    var ok = true
    for i in 0..<n:
      for j in 0..<i:
        if c[(i + a) mod n][j] != c[(j + a) mod n][i]:
          ok = false
    if ok:
      ans += 1
  echo ans * n
main()
