import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let a, b, c, d, e, f = read().parseInt

  # dp[i]: x * c + y * d = i にできるか
  var dp = newSeq[bool](f + 1)
  dp[0] = true
  for i in 0..<f:
    for j in @[c, d]:
      if i + j <= f:
        dp[i + j] = dp[i + j] or dp[i]
  var
    tot = a * 100
    sugar = 0
  for i in 0..(f div (a * 100)):
    for j in 0..(f div (b * 100)):
      let
        w1 = i * a * 100
        w2 = j * b * 100
      for k in 1..f:
        if dp[k] and w1 + w2 + k <= f and (w1 + w2) * e >= k * 100:
          # sugar/tot < k/(w1 + w2 + k)
          if sugar * (w1 + w2 + k) < k * tot:
            tot = w1 + w2 + k
            sugar = k
  echo tot, " ", sugar


main()
