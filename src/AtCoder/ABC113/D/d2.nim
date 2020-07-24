import strutils, sequtils

const mo = 1_000_000_000 + 7
proc add(a: var int64, b: int64) =
  a = (a mod mo + b mod mo) mod mo

proc main() =
  var h, w, k: int
  (h, w, k) = stdin.readLine.strip.split.map(parseInt)

  var fib = newSeq[int64](w + 1)
  fib[0] = 1
  fib[1] = 1
  for i in 2..w:
    fib[i] = fib[i - 1] + fib[i - 2] mod mo
  var dp = newSeq[int64](w + 1)
  dp[1] = 1
  for q in 0..<h:
    var nxt = newSeq[int64](w + 1)
    for j in 1..w:
      if j >= 2: # j --> j - 1
        add(nxt[j - 1], dp[j] * fib[j - 2] mod mo * fib[w - j] mod mo)
      # j --> j
      add(nxt[j], dp[j] * fib[j - 1] mod mo * fib[w - j] mod mo)
      if j + 1 <= w: # j --> j + 1
        add(nxt[j + 1], dp[j] * fib[j - 1] mod mo * fib[w - j - 1] mod mo)
    swap(dp, nxt)
  echo dp[k]
main()
