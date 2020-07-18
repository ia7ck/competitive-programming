import strutils, sequtils, tables

proc chmax(a: var int, b: int) =
  if a < b: a = b

proc main() =
  let
    args = stdin.readLine.strip.split.map(parseInt)
    (n, m, y, z) = (args[0], args[1], args[2], args[3])
    cp = (0..<m).mapIt(stdin.readLine.strip.split)
    s = stdin.readLine.strip
  var
    map = initTable[char, int]()
    base_point = newSeq[int](m)
  for it in cp:
    let ch = it[0][0]
    if not map.hasKey(ch):
      map[ch] = map.len
    base_point[map[ch]] = it[1].parseInt
  const inf = 1000000000
  var dp = newSeqWith(m + 1, newSeqWith(1 shl m, -inf))
  dp[m][0] = 0
  for ch in s:
    var nxt = newSeqWith(m + 1, newSeqWith(1 shl m, -inf))
    for i in 0..m:
      for j in 0..<(1 shl m):
        if dp[i][j] > -inf:
          let
            color = map[ch]
            used = j or (1 shl color)
          var ad = base_point[color]
          if i == color:
            ad += y
          chmax(nxt[color][used], dp[i][j] + ad)
          chmax(nxt[i][j], dp[i][j])
    dp.swap(nxt)
  var ans = -inf
  for i in 0..m:
    for j in 0..<(1 shl m):
      var score = dp[i][j]
      if j == (1 shl m) - 1:
        score += z
      chmax(ans, score)
  echo ans
main()
